import pandas as pd

# Sample DataFrames
data1 = {'A': [1, 2, 3], 'B': ['X', 'Y', 'Z']}
data2 = {'A': [2, 3, 4], 'B': ['Y', 'Z', 'W']}

df1 = pd.DataFrame(data1)
df2 = pd.DataFrame(data2)

# Find the intersection of the two DataFrames based on common columns 'A' and 'B'
intersection_df = pd.merge(df1, df2, on=['A', 'B'])

print(intersection_df)

