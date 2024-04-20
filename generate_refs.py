from esv_ranges import passage_data

psalms_data = next(filter(lambda x: x[0] == 'Psalms' if x else None, passage_data))

psalms_references = []

for chapter in range(1, psalms_data[1] + 1):  # Psalms has 150 chapters
    num_verses_in_chapter = psalms_data[2][chapter]  # Number of verses in each chapter
    verse = 1  # Reset verse count for each chapter
    psalms_references.append(f'"{psalms_data[0]}: {chapter}:{verse}-{verse+num_verses_in_chapter-1}",')

# Printing the list of references
for reference in psalms_references:
    print(reference)