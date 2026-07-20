class Solution:
    def dailyTemperatures(self, temperatures: list[int]) -> list[int]:
        answer = [0] * len(temperatures)
        unresolved = []

        for day, temperature in enumerate(temperatures):
            while unresolved and temperatures[unresolved[-1]] < temperature:
                earlier_day = unresolved.pop()
                answer[earlier_day] = day - earlier_day

            unresolved.append(day)

        return answer