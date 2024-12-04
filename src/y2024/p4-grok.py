import sys

def count_xmas(grid):
    directions = [(0,1), (1,0), (1,1), (-1,1), (0,-1), (-1,0), (-1,-1), (1,-1)] # All directions including diagonals
    count = 0
    
    def is_valid(y, x):
        return 0 <= y < len(grid) and 0 <= x < len(grid[0])

    for y in range(len(grid)):
        for x in range(len(grid[y])):
            for dy, dx in directions:
                for i in range(4):  # length of "XMAS"
                    ny, nx = y + dy*i, x + dx*i
                    if not is_valid(ny, nx) or grid[ny][nx] != 'XMAS'[i]:
                        break
                else:  # If we've checked all four characters without breaking
                    count += 1

    return count

# Here we assume the grid is a list of strings where each string represents a row in the puzzle
# grid = text.split('\n')
grid = sys.stdin.readlines()
print("Number of 'XMAS' occurrences:", count_xmas(grid))
