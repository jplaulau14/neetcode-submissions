class Solution:
    def carFleet(
        self, target: int, position: list[int], speed: list[int]
    ) -> int:
        cars = sorted(zip(position, speed), reverse=True)
        fleets = 0
        latest_distance = 0
        latest_speed = 1

        for car_position, car_speed in cars:
            distance = target - car_position

            if (
                fleets == 0
                or distance * latest_speed
                > latest_distance * car_speed
            ):
                fleets += 1
                latest_distance = distance
                latest_speed = car_speed

        return fleets