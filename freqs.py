import numpy as np
import pandas as pd
import math

freqs = pd.read_csv("./unigram_freq.csv")
words = pd.read_csv("./allowed_words.txt")

print(freqs.head(), "\n======\n", words.head())

freqs = freqs[freqs["word"].str.len() == 5]
freqs = freqs[freqs["word"].isin(words["aahed"])]

freqs["log"] = freqs["count"].apply(lambda x: math.log2(x))
total = freqs["log"].sum()
print(total)
freqs["probability"] = freqs["log"].apply(lambda x: x / total)

print(freqs.head(), "\n==\n", freqs.tail())

final = freqs[["probability", "word"]]
print(final.head())

final.to_csv("freqs.csv", encoding="utf-8", index=False)
