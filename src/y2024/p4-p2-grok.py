def count_x_mas(grid):
    count = 0
    for y in range(len(grid) - 2):  # minus 2 because we need 3 rows for an X
        for x in range(len(grid[0]) - 2):  # minus 2 for the same reason
            if (grid[y][x] in 'MAS' and
                grid[y+1][x+1] in 'MAS' and
                grid[y+2][x] in 'MAS' and
                grid[y+1][x] in 'MAS' and
                grid[y+1][x+2] in 'MAS'):
                
                # Check forward and backward for both MAS
                if (grid[y][x] + grid[y+1][x+1] + grid[y+2][x] == 'MAS' and 
                    grid[y+1][x] + grid[y+1][x+1] + grid[y+1][x+2] == 'MAS') or \
                   (grid[y][x] + grid[y+1][x+1] + grid[y+2][x] == 'SAM' and 
                    grid[y+1][x] + grid[y+1][x+1] + grid[y+1][x+2] == 'SAM') or \
                   (grid[y][x] + grid[y+1][x+1] + grid[y+2][x] == 'MAS' and 
                    grid[y+1][x] + grid[y+1][x+1] + grid[y+1][x+2] == 'SAM') or \
                   (grid[y][x] + grid[y+1][x+1] + grid[y+2][x] == 'SAM' and 
                    grid[y+1][x] + grid[y+1][x+1] + grid[y+1][x+2] == 'MAS'):
                    count += 1
    return count

# Read from stdin
import sys
lines = [line.strip() for line in sys.stdin]
grid = [line for line in lines if line]  # Exclude any empty lines

print("Number of X-MAS occurrences:", count_x_mas(grid))