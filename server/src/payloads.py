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
