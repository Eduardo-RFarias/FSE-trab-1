import asyncio
from enum import Enum
from typing import Any


class ClientId(Enum):
    GROUND_FLOOR = 0
    FIRST_FLOOR = 1
    SECOND_FLOOR = 2

    @classmethod
    def from_str(cls, value: str) -> "ClientId":
        if value == "ground_floor":
            return cls.GROUND_FLOOR
        if value == "first_floor":
            return cls.FIRST_FLOOR
        if value == "second_floor":
            return cls.SECOND_FLOOR

        raise ValueError(f"Invalid value {value}")


class Clients:
    def __init__(self):
        self.ground_floor: str | None = None
        self.first_floor: str | None = None
        self.second_floor: str | None = None

    def get_client_id_from_sid(self, sid: str) -> ClientId | None:
        if self.ground_floor == sid:
            return ClientId.GROUND_FLOOR
        if self.first_floor == sid:
            return ClientId.FIRST_FLOOR
        if self.second_floor == sid:
            return ClientId.SECOND_FLOOR

        return None

    def __getitem__(self, key: ClientId) -> str | None:
        if key == ClientId.GROUND_FLOOR.value:
            return self.ground_floor
        elif key == ClientId.FIRST_FLOOR.value:
            return self.first_floor
        elif key == ClientId.SECOND_FLOOR.value:
            return self.second_floor
        else:
            raise KeyError(f"Invalid key {key}")

    def __setitem__(self, key: ClientId, value: str | None):
        if key == ClientId.GROUND_FLOOR:
            self.ground_floor = value
        elif key == ClientId.FIRST_FLOOR:
            self.first_floor = value
        elif key == ClientId.SECOND_FLOOR:
            self.second_floor = value
        else:
            raise KeyError(f"Invalid key {key}")


class Car:
    def __init__(self, id: int, arrived_at: int):
        self.id = id
        self.arrived_at = arrived_at

    def calculate_fee(self, left_at: int) -> float:
        return (left_at - self.arrived_at) * 0.1


class ParkingSpaceType(Enum):
    NORMAL = 1
    HANDICAPPED = 2
    Elderly = 3


class ParkingSpace:
    def __init__(self, space_type: ParkingSpaceType):
        self.space_type: ParkingSpaceType = space_type
        self.car: Car | None = None

    def park(self, car: Car):
        if self.car is not None:
            raise ValueError("Parking space is already occupied")

        self.car = car

    def unpark(self) -> Car:
        car = self.car

        if car is None:
            raise ValueError("Parking space is already empty")

        self.car = None
        return car

    def is_empty(self) -> bool:
        return self.car is None


class ParkingFloor:
    def __init__(self):
        self.spaces: list[ParkingSpace] = []

        self.lock = asyncio.Lock()
        self.number_of_cars = 0

        self.spaces.append(ParkingSpace(ParkingSpaceType.HANDICAPPED))

        for _ in range(2):
            self.spaces.append(ParkingSpace(ParkingSpaceType.Elderly))

        for _ in range(5):
            self.spaces.append(ParkingSpace(ParkingSpaceType.NORMAL))

    async def park(self, car: Car, space: int):
        async with self.lock:
            self.spaces[space].park(car)
            self.number_of_cars += 1

    async def unpark(self, space: int) -> Car:
        async with self.lock:
            car = self.spaces[space].unpark()
            self.number_of_cars -= 1
            return car

    def is_full(self) -> bool:
        return self.number_of_cars == 8


class ParkingLot:
    def __init__(self):
        self.floors: list[ParkingFloor] = [
            ParkingFloor(),
            ParkingFloor(),
            ParkingFloor(),
        ]

        self.number_of_cars = 0
        self.lock = asyncio.Lock()

    async def park(self, car: Car, floor: int, space: int):
        async with self.lock:
            await self.floors[floor].park(car, space)
            self.number_of_cars += 1

    async def unpark(self, floor: int, space: int) -> Car:
        async with self.lock:
            car = await self.floors[floor].unpark(space)
            self.number_of_cars -= 1
            return car

    def is_full(self) -> bool:
        return self.number_of_cars == 24
