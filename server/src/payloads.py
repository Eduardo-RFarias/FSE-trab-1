from server.src.models import ClientId


class ParkingSpaceModifiedPayload:
    def __init__(self, parking_space: int, timestamp: int):
        self.parking_space = parking_space
        self.timestamp = timestamp

    @staticmethod
    def from_dict(dict: dict) -> "ParkingSpaceModifiedPayload":
        return ParkingSpaceModifiedPayload(
            parking_space=dict["parking_space"],
            timestamp=dict["timestamp"],
        )


class CloseFloorPayload:
    def __init__(self, client_id: str):
        self.client_id: ClientId = ClientId.from_str(client_id)

    @staticmethod
    def from_dict(dict: dict) -> "CloseFloorPayload":
        return CloseFloorPayload(
            client_id=dict["client_id"],
        )
