import asyncio

import socketio
from aiohttp import web

from .central_database import car_id, clients, parking_lot
from .constants import (
    CAR_ARRIVED_EVENT,
    CAR_DEPARTED_EVENT,
    CLIENT_ID_HEADER,
    CLIENT_ID_HEADER_FROM_SOCKET,
    CLOSE_FLOOR_EVENT,
    CLOSE_PARKING_LOT_EVENT,
    OPEN_FLOOR_EVENT,
    OPEN_PARKING_LOT_EVENT,
    ORDER_TO_CLOSE_FLOOR,
    ORDER_TO_CLOSE_PARKING_LOT,
    PARKING_LOT_STATE_EVENT,
)
from .models import Car, ClientId
from .payloads import CloseFloorPayload, ParkingSpaceModifiedPayload

socket = socketio.AsyncServer()

socket_lock = asyncio.Lock()


@socket.event
async def connect(sid: str, environ: dict[str, str]):
    x_client_id = environ.get(CLIENT_ID_HEADER_FROM_SOCKET)

    if x_client_id is None:
        raise ValueError(f"Missing {CLIENT_ID_HEADER} header")

    client_id = ClientId.from_str(x_client_id)

    await clients.set_item(client_id, sid)

    async with socket_lock:
        await socket.enter_room(sid, client_id.name)
        print(f"connected {sid} as {client_id}")
        await socket.emit(PARKING_LOT_STATE_EVENT, room=client_id.name)


@socket.event
async def disconnect(sid):
    client_id = await clients.get_client_id_from_sid(sid)

    if client_id is not None:
        await clients.set_item(client_id, None)

        async with socket_lock:
            await socket.close_room(client_id.name)

        print(f"disconnected {sid} from {client_id}")
    else:
        print(f"disconnected {sid}")


@socket.on(CAR_ARRIVED_EVENT)  # type: ignore
async def car_arrived(sid: str, payload: dict):
    client_id = await clients.get_client_id_from_sid(sid)

    if client_id is None:
        raise ValueError(f"Unknown client {sid}")

    floor = client_id.value
    dto = ParkingSpaceModifiedPayload.from_dict(payload)

    car = Car(id=await car_id.increment_and_get(), arrived_at=dto.timestamp)
    await parking_lot.park(car, floor, dto.parking_space)

    if await parking_lot.is_full():
        async with socket_lock:
            await socket.emit(CLOSE_PARKING_LOT_EVENT, room=client_id.name)

    if await parking_lot.is_floor_full(floor):
        async with socket_lock:
            await socket.emit(CLOSE_FLOOR_EVENT, room=client_id.name)


@socket.on(CAR_DEPARTED_EVENT)  # type: ignore
async def car_departed(sid: str, payload: dict):
    client_id = await clients.get_client_id_from_sid(sid)

    if client_id is None:
        raise ValueError(f"Unknown client {sid}")

    floor = client_id.value
    dto = ParkingSpaceModifiedPayload.from_dict(payload)

    parking_lot_was_full = await parking_lot.is_full()
    floor_was_full = await parking_lot.is_floor_full(floor)

    car = await parking_lot.unpark(floor, dto.parking_space)

    fee = car.calculate_fee(dto.timestamp)

    print(f"Car {car._id}'s fee is {fee}")

    if parking_lot_was_full:
        async with socket_lock:
            await socket.emit(OPEN_PARKING_LOT_EVENT, room=client_id.name)

    if floor_was_full:
        async with socket_lock:
            await socket.emit(OPEN_FLOOR_EVENT, room=client_id.name)


@socket.on(ORDER_TO_CLOSE_PARKING_LOT)  # type: ignore
async def order_to_close_parking_lot(sid: str, payload: dict):
    client_id = ClientId.GROUND_FLOOR

    async with socket_lock:
        await socket.emit(CLOSE_PARKING_LOT_EVENT, room=client_id.name)


@socket.on(ORDER_TO_CLOSE_FLOOR)  # type: ignore
async def order_to_close_floor(sid: str, payload: dict):
    client_id = CloseFloorPayload.from_dict(payload).client_id

    async with socket_lock:
        await socket.emit(CLOSE_FLOOR_EVENT, room=client_id.name)


app = web.Application()
socket.attach(app)
