use crate::models::{
    client::ClientId,
    parking_lot::{Floor, Spot, SpotType, Vehicle},
};
use crate::socket::payloads::{
    FloorDataPayload, ParkingLotDataPayload, SpotDataPayload, VehicleDataPayload,
};
use rusqlite::{named_params, Connection, Error};
use std::{
    collections::HashMap,
    fs,
    sync::{Arc, Mutex},
};

pub struct Database {
    connection: Connection,
    pub clients: HashMap<String, ClientId>,
}

impl Database {
    pub fn new() -> Arc<Mutex<Self>> {
        // create db folder if it doesn't exist
        fs::create_dir_all("./db").unwrap();

        let connection = Connection::open("./db/parking_lot.db").unwrap();

        let instance = Self {
            connection,
            clients: HashMap::with_capacity(3),
        };

        instance.initialize_database_state();

        Arc::new(Mutex::new(instance))
    }

    fn initialize_database_state(&self) {
        // Check if the tables already exist and if they do, return early
        let tables_count: i32 = self
            .connection
            .query_row(
                "
                SELECT COUNT(*) 
                FROM sqlite_master 
                WHERE type='table' AND name IN ('parking_lot', 'vehicle', 'parking_floor', 'parking_spot', 'car_exit');",
                [],
                |row| row.get(0),
            )
            .unwrap();

        if tables_count == 5 {
            return;
        }

        // Create the tables and insert the initial data
        self.connection
            .execute_batch(
                "
                CREATE TABLE parking_lot (
                    is_closed BOOLEAN NOT NULL
                );

                CREATE TABLE vehicle (
                    id INTEGER NOT NULL PRIMARY KEY,
                    entry_time BIGINT NOT NULL
                );

                CREATE TABLE parking_floor (
                    floor_number INTEGER NOT NULL PRIMARY KEY CHECK (floor_number IN (0, 1, 2)),
                    is_closed BOOLEAN NOT NULL
                );
                
                CREATE TABLE parking_spot (
                    floor_number INTEGER NOT NULL,
                    spot_number INTEGER NOT NULL CHECK (spot_number IN (0, 1, 2, 3, 4, 5, 6, 7)),
                    spot_type INTEGER NOT NULL CHECK (spot_type IN (0, 1, 2)),
                    parked_vehicle_id INTEGER UNIQUE,
                    PRIMARY KEY (floor_number, spot_number),
                    FOREIGN KEY (parked_vehicle_id) REFERENCES vehicle(id),
                    FOREIGN KEY (floor_number) REFERENCES parking_floor(floor_number)
                );

                CREATE TABLE car_exit (
                    id INTEGER NOT NULL PRIMARY KEY,
                    exit_time BIGINT NOT NULL,
                    FOREIGN KEY (id) REFERENCES vehicle(id)
                );

                -- Insert the initial state of the parking lot
                INSERT INTO parking_lot(is_closed) VALUES (0);


                -- Insert the initial state of the parking floors
                INSERT INTO parking_floor(floor_number, is_closed) VALUES
                    (0, 0),
                    (1, 0),
                    (2, 0);
                
                -- Insert 8 parking spots for the first floor
                INSERT INTO parking_spot(floor_number, spot_number, spot_type) VALUES
                    (0, 0, 1),
                    (0, 1, 2),
                    (0, 2, 2),
                    (0, 3, 0),
                    (0, 4, 0),
                    (0, 5, 0),
                    (0, 6, 0),
                    (0, 7, 0);
                                
                -- Insert 8 parking spots for the second floor
                INSERT INTO parking_spot(floor_number, spot_number, spot_type) VALUES
                    (1, 0, 1),
                    (1, 1, 2),
                    (1, 2, 2),
                    (1, 3, 0),
                    (1, 4, 0),
                    (1, 5, 0),
                    (1, 6, 0),
                    (1, 7, 0);
                
                -- Insert 8 parking spots for the third floor
                INSERT INTO parking_spot(floor_number, spot_number, spot_type) VALUES
                    (2, 0, 1),
                    (2, 1, 2),
                    (2, 2, 2),
                    (2, 3, 0),
                    (2, 4, 0),
                    (2, 5, 0),
                    (2, 6, 0),
                    (2, 7, 0);",
            )
            .unwrap();

        println!("Database initialized successfully!");
    }

    pub fn get_floor(&self, floor_number: i32) -> Result<Floor, Error> {
        let mut stmt = self.connection.prepare(
            "
            SELECT
                ps.spot_number,
                ps.spot_type,
                v.id as vehicle_id,
                v.entry_time as vehicle_entry_type
            from
                parking_spot ps
            LEFT JOIN vehicle v ON
                ps.parked_vehicle_id = v.id
            WHERE
                ps.floor_number = :floor_number;",
        )?;

        let spots = stmt.query_map(
            named_params! {
                ":floor_number": floor_number,
            },
            |row| {
                let spot_number: i32 = row.get(0)?;
                let spot_type: SpotType = row.get(1)?;
                let parked_vehicle_id: Option<i32> = row.get(2)?;
                let parked_vehicle_entry_time: Option<i64> = row.get(3)?;

                let mut spot = Spot {
                    spot_number,
                    spot_type,
                    parked_vehicle: None,
                };

                if let (Some(parked_vehicle_id), Some(parked_vehicle_entry_time)) =
                    (parked_vehicle_id, parked_vehicle_entry_time)
                {
                    spot.parked_vehicle = Some(Vehicle {
                        id: parked_vehicle_id,
                        entry_time: parked_vehicle_entry_time,
                    });
                }

                Ok(spot)
            },
        )?;

        let mut floor = Floor {
            floor_number,
            spots: Vec::with_capacity(8),
        };

        for spot in spots {
            floor.spots.push(spot?);
        }

        Ok(floor)
    }

    fn get_spot(&self, floor_number: i32, spot_number: i32) -> Result<Spot, Error> {
        let mut stmt = self.connection.prepare(
            "
            SELECT
                ps.spot_type,
                v.id as vehicle_id,
                v.entry_time as vehicle_entry_time
            FROM
                parking_spot ps
            LEFT JOIN vehicle v ON
                ps.parked_vehicle_id = v.id
            WHERE
                ps.floor_number = :floor_number AND ps.spot_number = :spot_number;",
        )?;

        stmt.query_row(
            named_params! {
                ":floor_number": floor_number,
                ":spot_number": spot_number,
            },
            |row| {
                let spot_type: SpotType = row.get(0)?;
                let parked_vehicle_id: Option<i32> = row.get(1)?;
                let parked_vehicle_entry_time: Option<i64> = row.get(2)?;

                let mut spot = Spot {
                    spot_number,
                    spot_type,
                    parked_vehicle: None,
                };

                if let (Some(parked_vehicle_id), Some(parked_vehicle_entry_time)) =
                    (parked_vehicle_id, parked_vehicle_entry_time)
                {
                    spot.parked_vehicle = Some(Vehicle {
                        id: parked_vehicle_id,
                        entry_time: parked_vehicle_entry_time,
                    });
                }

                Ok(spot)
            },
        )
    }

    pub fn park_vehicle(
        &mut self,
        vehicle_entry_time: i64,
        floor_number: i32,
        spot_number: i32,
    ) -> Result<(), Error> {
        let tx = self.connection.transaction()?;

        let car_id = tx
            .prepare("INSERT INTO vehicle(entry_time) VALUES (:entry_time);")?
            .insert(named_params! {
                ":entry_time": vehicle_entry_time,
            })?;

        tx.prepare(
            "
            UPDATE 
                parking_spot SET parked_vehicle_id = :car_id 
            WHERE 
                floor_number = :floor_number AND spot_number = :spot_number;",
        )?
        .execute(named_params! {
            ":car_id": car_id,
            ":floor_number": floor_number,
            ":spot_number": spot_number,
        })?;

        tx.commit()?;

        Ok(())
    }

    pub fn unpark_vehicle(
        &mut self,
        floor_number: i32,
        spot_number: i32,
        exit_time: i64,
    ) -> Result<(), Error> {
        let vehicle = self.get_spot(floor_number, spot_number)?.parked_vehicle;

        if let Some(vehicle) = vehicle {
            let tx = self.connection.transaction()?;

            tx.prepare(
                "
                UPDATE 
                    parking_spot SET parked_vehicle_id = NULL 
                WHERE 
                    floor_number = :floor_number AND spot_number = :spot_number;",
            )?
            .execute(named_params! {
                ":floor_number": floor_number,
                ":spot_number": spot_number,
            })?;

            tx.prepare("INSERT INTO car_exit(id, exit_time) VALUES (:vehicle_id, :exit_time);")?
                .execute(named_params! {
                    ":vehicle_id": vehicle.id,
                    ":exit_time": exit_time,
                })?;

            tx.commit()?;

            Ok(())
        } else {
            Err(Error::QueryReturnedNoRows)
        }
    }

    pub fn parking_lot_is_full(&self) -> Result<bool, Error> {
        let mut stmt = self.connection.prepare(
            "
            SELECT
                COUNT(*)
            FROM
                parking_spot ps
            INNER JOIN vehicle v ON
                ps.parked_vehicle_id = v.id;",
        )?;

        let num_of_cars: i32 = stmt.query_row([], |row| row.get(0))?;

        Ok(num_of_cars >= 24)
    }

    pub fn floor_is_full(&self, floor_number: i32) -> Result<bool, Error> {
        let mut stmt = self.connection.prepare(
            "
            SELECT
                COUNT(*)
            FROM
                parking_spot ps
            INNER JOIN vehicle v ON
                ps.parked_vehicle_id = v.id
            WHERE
                ps.floor_number = :floor_number;",
        )?;

        let num_of_cars: i32 = stmt.query_row(
            named_params! {
                ":floor_number": floor_number,
            },
            |row| row.get(0),
        )?;

        Ok(num_of_cars >= 8)
    }

    pub fn close_parking_lot(&mut self) -> Result<(), Error> {
        self.connection.execute(
            "
            UPDATE 
                parking_lot 
            SET 
                is_closed = 1;",
            [],
        )?;

        Ok(())
    }

    pub fn close_floor(&mut self, floor_number: i32) -> Result<(), Error> {
        self.connection.execute(
            "
            UPDATE 
                parking_floor 
            SET 
                is_closed = 1 
            WHERE 
                floor_number = :floor_number;",
            named_params! {
                ":floor_number": floor_number,
            },
        )?;

        Ok(())
    }

    pub fn open_parking_lot(&mut self) -> Result<(), Error> {
        self.connection.execute(
            "
            UPDATE 
                parking_lot 
            SET 
                is_closed = 0;",
            [],
        )?;

        Ok(())
    }

    pub fn open_floor(&mut self, floor_number: i32) -> Result<(), Error> {
        self.connection.execute(
            "
            UPDATE 
                parking_floor 
            SET 
                is_closed = 0 
            WHERE 
                floor_number = :floor_number;",
            named_params! {
                ":floor_number": floor_number,
            },
        )?;

        Ok(())
    }

    pub fn is_floor_closed(&self, floor_number: i32) -> Result<bool, Error> {
        let is_closed: i32 = self.connection.query_row(
            "
            SELECT 
                is_closed 
            FROM 
                parking_floor 
            WHERE 
                floor_number = :floor_number;",
            named_params! {
                ":floor_number": floor_number,
            },
            |row| row.get(0),
        )?;

        Ok(is_closed == 1)
    }

    pub fn is_parking_lot_closed(&self) -> Result<bool, Error> {
        let is_closed: i32 =
            self.connection
                .query_row("SELECT is_closed FROM parking_lot;", [], |row| row.get(0))?;

        Ok(is_closed == 1)
    }

    pub fn get_exited_vehicles(&self) -> Result<Vec<VehicleDataPayload>, Error> {
        let mut stmt = self.connection.prepare(
            "
            SELECT
                v.id,
                v.entry_time,
                ce.exit_time
            FROM
                vehicle v
            INNER JOIN car_exit ce ON
                v.id = ce.id
            ORDER BY
                ce.exit_time DESC;",
        )?;

        let vehicles = stmt.query_map([], |row| {
            let id: i32 = row.get(0)?;
            let entry_time: i64 = row.get(1)?;
            let exit_time: i64 = row.get(2)?;

            Ok(VehicleDataPayload {
                id,
                entry_time,
                exit_time: Some(exit_time),
            })
        })?;

        let mut exited_vehicles = Vec::new();

        for vehicle in vehicles {
            exited_vehicles.push(vehicle?);
        }

        Ok(exited_vehicles)
    }

    pub fn get_parking_lot_state(&self) -> Result<ParkingLotDataPayload, Error> {
        let mut data = ParkingLotDataPayload {
            floors: Vec::with_capacity(3),
            exited_vehicles: self.get_exited_vehicles()?,
            is_closed: self.is_parking_lot_closed()?,
        };

        for floor_number in 0..3 {
            let mut floor_data = FloorDataPayload {
                is_closed: self.is_floor_closed(floor_number)?,
                spots: Vec::with_capacity(8),
            };

            let floor = self.get_floor(floor_number)?;

            for spot in floor.spots {
                let spot_data = SpotDataPayload {
                    spot_type: spot.spot_type as i32,
                    parked_vehicle: spot.parked_vehicle.map(|vehicle| VehicleDataPayload {
                        id: vehicle.id,
                        entry_time: vehicle.entry_time,
                        exit_time: None,
                    }),
                };

                floor_data.spots.push(spot_data);
            }

            data.floors.push(floor_data);
        }

        Ok(data)
    }

    pub fn reset_parking_lot(&mut self) -> Result<(), Error> {
        let tx = self.connection.transaction()?;

        tx.execute("UPDATE parking_spot SET parked_vehicle_id = NULL;", [])?;
        tx.execute("DELETE FROM car_exit;", [])?;
        tx.execute("DELETE FROM vehicle;", [])?;
        tx.execute("UPDATE parking_lot SET is_closed = 0;", [])?;
        tx.execute("UPDATE parking_floor SET is_closed = 0;", [])?;

        tx.commit()?;

        Ok(())
    }
}
