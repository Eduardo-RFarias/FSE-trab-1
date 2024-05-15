import asyncio

import socketio
from aiohttp import web

from .central_database import car_id, clients, parking_lot
from .models import Car, ClientId
from .payloads import ParkingSpaceModifiedPayload

sio = socketio.AsyncServer()

lock = asyncio.Lock()


@sio.event
async def connect(sid: str, environ: dict[str, str]):
    x_client_id = environ.get("HTTP_X_CLIENT_ID")

    if x_client_id is None:
        raise ValueError("Missing X-Client-Id header")

    client_id = ClientId.from_str(x_client_id)

    async with lock:
        clients[client_id] = sid
        await sio.enter_room(sid, client_id.name)

    print(f"connected {sid} as {client_id}")


@sio.event
async def disconnect(sid):
    client_id = clients.get_client_id_from_sid(sid)

    if client_id is not None:
        async with lock:
            clients[client_id] = None
            await sio.close_room(sid)
            print(f"disconnected {sid} from {client_id}")
    else:
        print(f"disconnected {sid}")


@sio.event
async def car_arrived(sid: str, payload: dict):
    client_id = clients.get_client_id_from_sid(sid)

    if client_id is None:
        raise ValueError(f"Unknown client {sid}")

    floor = client_id.value
    dto = ParkingSpaceModifiedPayload.from_dict(payload)

    car = Car(id=await car_id.increment_and_get(), arrived_at=dto.timestamp)
    await parking_lot.park(car, floor, dto.parking_space)

    print(f"Car {car.id} parked at {floor} floor parking space {dto.parking_space}")

    if parking_lot.is_full():
        print("Parking lot is now full")
        async with lock:
            await sio.emit("close_parking_lot", room=client_id.name)

    if parking_lot.floors[floor].is_full():
        print(f"{floor} floor is now full")
        async with lock:
            await sio.emit("close_floor", room=client_id.name)


@sio.event
async def car_departed(sid: str, payload: dict):
    client_id = clients.get_client_id_from_sid(sid)

    if client_id is None:
        raise ValueError(f"Unknown client {sid}")

    floor = client_id.value
    dto = ParkingSpaceModifiedPayload.from_dict(payload)

    async with lock:
        parking_lot_was_full = parking_lot.is_full()
        floor_was_full = parking_lot.floors[floor].is_full()

        car = await parking_lot.unpark(floor, dto.parking_space)

        fee = car.calculate_fee(dto.timestamp)

        print(
            f"Car {car.id} left from {floor} floor, space {dto.parking_space}.", end=" "
        )
        print(f"Fee: {fee}")

        if parking_lot_was_full:
            print("Parking lot is no longer full")
            await sio.emit("open_parking_lot", room=client_id.name)

        if floor_was_full:
            print(f"{floor} floor is no longer full")
            await sio.emit("open_floor", room=client_id.name)


app = web.Application()
sio.attach(app)
