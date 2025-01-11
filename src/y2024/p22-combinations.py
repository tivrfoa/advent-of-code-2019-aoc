import itertools

def generate_combinations():
  """
  Generates all combinations with 4 numbers in the range -9 to 9, inclusive.

  Returns:
    A list of tuples, where each tuple represents a combination of 4 numbers.
  """
  numbers = list(range(-9, 10))  # Create a list of numbers from -9 to 9
  combinations = list(itertools.product(numbers, repeat=4)) 
  return combinations

# Get all combinations
all_combinations = generate_combinations()

# Print the number of combinations
print(f"Number of combinations: {len(all_combinations)}") 

# Print the first 10 combinations (for example)
print("First 10 combinations:")
for i in range(100):
  print(all_combinations[i])
