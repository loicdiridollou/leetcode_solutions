"""Problem 2696 - 2696. Minimum String Length After Removing Substrings."""


class Solution:
    """Class for the problem's solution."""

    def minLength(self, s: str) -> int:
        """Compute length of string after removing substring."""
        stack = []

        for i in s:
            if i == "B" and stack and stack[-1] == "A":
                stack.pop()
            elif i == "D" and stack and stack[-1] == "C":
                stack.pop()
            else:
                stack.append(i)

        return len(stack)

    def minLength_naive(self, s: str) -> int:
        """Compute length of string after removing substring naively."""
        i = 0
        while i < len(s) - 1:
            if s[i : i + 2] == "AB":
                s = s[:i] + s[i + 2 :]
                i = 0
            elif s[i : i + 2] == "CD":
                s = s[:i] + s[i + 2 :]
                i = 0
            else:
                i += 1
        return len(s)


def test_solution():
    """Test problem solving."""
    sol = Solution()
    assert sol.minLength("ABFCACDB") == 2
    assert sol.minLength("ACBBD") == 5
