"""Problem 1513 - Number of Substrings with Only 1s."""


class Solution:
    """Class for the problem's solution."""

    def numSub(self, s: str) -> int:
        """Count the number of subsequences with just 1s."""
        return int(
            sum(map(lambda x: x * (x + 1) / 2, map(lambda x: len(x), s.split("0"))))
            % (1e9 + 7)
        )

    def numSub_naive(self, s: str) -> int:
        """Count the number of subsequences with just 1s naively."""
        lst = []
        i = 0
        while i < len(s):
            if s[i] == "0":
                i += 1
            else:
                j = i
                while j < len(s) and s[j] == "1":
                    j += 1
                lst.append(j - i)
                i = j

        cnt = 0
        for val in lst:
            for i in range(1, val + 1):
                cnt += val - i + 1
        return int(cnt % (1e9 + 7))


def test_solution():
    """Test problem solving."""
    sol = Solution()
    assert sol.numSub("0110111") == 9
    assert sol.numSub("101") == 2
