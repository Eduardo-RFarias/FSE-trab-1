import asyncio

from .models import Clients, ParkingLot


class AtomicCounter:
    def __init__(self):
        self._value = 0
        self._lock = asyncio.Lock()

    async def increment_and_get(self) -> int:
        async with self._lock:
            self._value += 1
            return self._value


car_id = AtomicCounter()


clients = Clients()
parking_lot = ParkingLot()
