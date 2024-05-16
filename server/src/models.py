import asyncio
from enum import Enum
from typing import Union


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
        self._ground_floor: Union[str, None] = None
        self._first_floor: Union[str, None] = None
        self._second_floor: Union[str, None] = None

        self.lock = asyncio.Lock()

    async def get_client_id_from_sid(self, sid: str) -> Union[ClientId, None]:
        async with self.lock:
            if self._ground_floor == sid:
                return ClientId.GROUND_FLOOR
            if self._first_floor == sid:
                return ClientId.FIRST_FLOOR
            if self._second_floor == sid:
                return ClientId.SECOND_FLOOR

            return None

    async def get_item(self, key: ClientId) -> Union[str, None]:
        async with self.lock:
            if key == ClientId.GROUND_FLOOR.value:
                return self._ground_floor
            elif key == ClientId.FIRST_FLOOR.value:
                return self._first_floor
            elif key == ClientId.SECOND_FLOOR.value:
                return self._second_floor
            else:
                raise KeyError(f"Invalid key {key}")

    async def set_item(self, key: ClientId, value: Union[str, None]):
        async with self.lock:
            if key == ClientId.GROUND_FLOOR:
                self._ground_floor = value
            elif key == ClientId.FIRST_FLOOR:
                self._first_floor = value
            elif key == ClientId.SECOND_FLOOR:
                self._second_floor = value
            else:
                raise KeyError(f"Invalid key {key}")


class Car:
    def __init__(self, id: int, arrived_at: int):
        self._id = id
        self._arrived_at = arrived_at

    def calculate_fee(self, left_at: int) -> float:
        # 0.1$ per minute
        return (left_at - self._arrived_at) // 60 * 0.1


class ParkingSpaceType(Enum):
    NORMAL = 1
    HANDICAPPED = 2
    Elderly = 3


class ParkingSpace:
    def __init__(self, space_type: ParkingSpaceType):
        self._space_type: ParkingSpaceType = space_type
        self._car: Union[Car, None] = None

    def park(self, car: Car):
        if self._car is not None:
            raise ValueError("Parking space is already occupied")

        self._car = car

    def unpark(self) -> Car:
        car = self._car

        if car is None:
            raise ValueError("Parking space is already empty")

        self._car = None
        return car

    def is_empty(self) -> bool:
        return self._car is None


class ParkingFloor:
    def __init__(self):
        self._spaces: list[ParkingSpace] = []
        self._number_of_cars = 0

        self._spaces.append(ParkingSpace(ParkingSpaceType.HANDICAPPED))

        for _ in range(2):
            self._spaces.append(ParkingSpace(ParkingSpaceType.Elderly))

        for _ in range(5):
            self._spaces.append(ParkingSpace(ParkingSpaceType.NORMAL))

    def park(self, car: Car, space: int):
        self._spaces[space].park(car)
        self._number_of_cars += 1

    def unpark(self, space: int) -> Car:
        car = self._spaces[space].unpark()
        self._number_of_cars -= 1
        return car

    def is_full(self) -> bool:
        return self._number_of_cars == 8


class ParkingLot:
    def __init__(self):
        self._floors: list[ParkingFloor] = [
            ParkingFloor(),
            ParkingFloor(),
            ParkingFloor(),
        ]

        self._number_of_cars = 0
        self._lock = asyncio.Lock()

    async def park(self, car: Car, floor: int, space: int):
        async with self._lock:
            self._floors[floor].park(car, space)
            self._number_of_cars += 1

    async def unpark(self, floor: int, space: int) -> Car:
        async with self._lock:
            car = self._floors[floor].unpark(space)
            self._number_of_cars -= 1
            return car

    async def is_full(self) -> bool:
        async with self._lock:
            return self._number_of_cars == 24

    async def is_floor_full(self, floor: int) -> bool:
        async with self._lock:
            return self._floors[floor].is_full()

    async def get_floor_as_bool_array(self, floor) -> list[bool]:
        async with self._lock:
            return [space.is_empty() for space in self._floors[floor]._spaces]
