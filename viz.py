import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt
import numpy as np
import matplotlib.colors as mcolors

# Load the similarity scores data
df = pd.read_csv('similarity_report.csv')

# Average similarity scores by book, chapter, and verse
verse_similarity = df.pivot_table(index=['Book ID', 'Chapter'], columns='Verse', values='Similarity', aggfunc='mean')

# Reversing the 'coolwarm' colormap
reversed_coolwarm = sns.color_palette("coolwarm", as_cmap=True).reversed()

# Customize the heatmap for better visual differentiation between chapters and verses
plt.figure(figsize=(20, 15))
ax = sns.heatmap(verse_similarity, annot=True, cmap=reversed_coolwarm , fmt=".2f", linewidths=.5, linecolor='gray', vmin=0.7, vmax=1)

# Adding custom gridlines for chapters over verses
# Get the number of verses per chapter to place bolder lines
chapters = df.groupby(['Book ID', 'Chapter']).size().reset_index(name='Counts')
verse_counts = chapters.groupby('Book ID').cumsum()['Counts']

# Draw thicker vertical lines at chapter boundaries
for count in verse_counts[:-1]:  # Exclude the last cumulative count
    ax.axvline(x=count, color='black', linestyle='-', linewidth=2)

# Labels and Title
plt.title('Average Similarity Scores per Chapter and Verse across Translations', fontsize=16)
plt.xlabel('Verse', fontsize=14)
plt.ylabel('Book ID - Chapter', fontsize=14)

# Adjust the tick labels for better readability
ax.set_yticklabels(ax.get_yticklabels(), rotation=0, fontsize=10)
ax.set_xticklabels(ax.get_xticklabels(), rotation=0, fontsize=10)

plt.tight_layout()
plt.show()
