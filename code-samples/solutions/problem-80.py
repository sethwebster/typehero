def diff_strings(old, new):
    m, n = len(old), len(new)
    dp = [[0] * (n + 1) for _ in range(m + 1)]

    # Build LCS length table
    for i in range(1, m + 1):
        for j in range(1, n + 1):
            if old[i-1] == new[j-1]:
                dp[i][j] = dp[i-1][j-1] + 1
            else:
                dp[i][j] = max(dp[i-1][j], dp[i][j-1])

    # Backtrack to build diff
    changes = []
    i, j = m, n

    while i > 0 or j > 0:
        if i > 0 and j > 0 and old[i-1] == new[j-1]:
            changes.append(('equal', old[i-1]))
            i -= 1
            j -= 1
        elif j > 0 and (i == 0 or dp[i][j-1] >= dp[i-1][j]):
            changes.append(('insert', new[j-1]))
            j -= 1
        else:
            changes.append(('delete', old[i-1]))
            i -= 1

    return list(reversed(changes))
