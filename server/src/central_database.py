import asyncio

from .models import Clients, ParkingLot


class AtomicCounter:
    def __init__(self):
        self.value = 0
        self.lock = asyncio.Lock()

    async def increment_and_get(self) -> int:
        async with self.lock:
            self.value += 1
            return self.value


car_id = AtomicCounter()


clients = Clients()
parking_lot = ParkingLot()
