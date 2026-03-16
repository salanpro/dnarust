## Calculating the molecular weight
$$
\sum Aminoacids - ((n-1) * H_2O)
$$

Formation of the peptide bond and its rupture by hydrolysis

## How Encoder and Decoder works
Encoder: Takes bytes → extracts bit pairs → maps to nucleotides

Decoder: Takes nucleotides → maps to bit pairs → reconstructs bytes

Each nucleotide represents 2 bits, since each byte has 8 bits you can represent 1 byte as 4 nucleotides
```
00 → A
01 → T
10 → G
11 → C
```
