import pandas as pd
import json

class BibleAphorismGenerator:
    def __init__(self, chapter, aphorisms, explanations, stories, references):
        self.chapter = chapter
        self.aphorisms = aphorisms
        self.explanations = explanations
        self.stories = stories
        self.references = references

    def to_json(self):
        """Convert model properties to a JSON string."""
        return json.dumps({
            "chapter": self.chapter,
            "aphorisms": self.aphorisms,
            "explanations": self.explanations,
            "stories": self.stories,
            "references": self.references
        }, indent=4)

df = pd.read_csv("similarity_report.csv", header=0).sort_values(by='Verse Key')
row_count = df.shape[0]
df.drop_duplicates(inplace=True)
new_row_count = df.shape[0]
if row_count != new_row_count:  # fix up duplicates due to the hastily way the extraction process was written (not removing files yet so we double load them)
    df.to_csv("similarity_report.csv", index=False)

# Count total verse keys
total_keys = len(df['Verse Key'])

# Count distinct verse keys
distinct_keys = df['Verse Key'].nunique()

# Print results
print(f"Total Verse Keys: {total_keys}")
print(f"Distinct Verse Keys: {distinct_keys}")

# Check for duplicates by comparing counts
if total_keys > distinct_keys:
    print("There are duplicates in the Verse Key column.\n\n")
else:
    print("No duplicates found in the Verse Key column.")

chapter = 7

# Provide me some aphorisms that are abstracted to real life concepts and adages from this bible chapter 
# In addition to explaining the bible verse, come up with a real life story too
# When you came up with the aphorisms were you able to identify a reference as far as chapter, verse range for each?
df2 = df[(df['Translation 1'] == 'World English Bible') & (df['Chapter'] == chapter)][['Text 1']]
script = '\n'.join(df2['Text 1'].tolist())

prompt = f"""
Objective:
Create a set of aphorisms, explanations, real-life stories, and verse references from a specific chapter in the Bible. Each element should be tied to the content and themes of the selected chapter, reflecting both spiritual insights and practical applications.

Bible Chapter:

    Book Name: Job
    Chapter Number: {chapter}

Tasks:

    Generate Aphorisms:
        Description: Develop insightful and thought-provoking aphorisms that abstract the key themes and moral lessons from the bible chapter verse.
        Output Requirements: List of aphorisms.

    Provide Explanations for Each Aphorism:
        Description: For each aphorism, provide a concise explanation that elaborates on the thought behind the aphorism and how it relates to the themes of the chapter.
        Output Requirements: Corresponding explanations that match each aphorism.

    Craft Real-Life Stories:
        Description: Develop long stories that illustrate the practical application of each aphorism in contemporary settings without mentioning God, demonstrating how these ancient wisdoms can be applied to modern life challenges.
        Output Requirements: A story for each aphorism showing its application in real life.

    Identify Specific Verse References:
        Description: For each aphorism, identify specific verses from the bible chapter verse that support or relate to the aphorism and its explanation.
        Output Requirements: Verse references for each aphorism.

Expected Format:

    The output should be a list of dictionaries. Each dictionary represents an aphorism along with its explanation, the full text of the referenced verses, the string 'World English Bible', a real-life story, and the specific verse references from the bible chapter verse. The dictionaries should have the following keys: aphorism, explanation, story, reference.

Example Output:

json

[
    {{
        "aphorism": "Fortune is a wheel; it turns endlessly, elevating and humbling.",
        "text": "Full text of referenced verses.",
        "explanation": "This reflects the unpredictable nature of life's circumstances, as seen in Job’s sudden loss despite his righteousness.",
        "story": "Consider a successful entrepreneur who faces a sudden downturn due to market changes, mirroring Job’s trials and testing their resilience and moral fiber.",
        "reference": "Job 1:1-3, 1:13-19",
        "translation": "World English Bible"
    }},
    {{
        "aphorism": "Virtue is tested in the fires of adversity.",
        "text": "Full text of referenced verses.",
        "explanation": "True character is revealed through challenges, as Job maintains his integrity despite his losses.",
        "story": "A community leader continues to advocate for the needy despite facing personal tragedies, showing deep commitment to her principles.",
        "reference": "Job 1:20-22",
        "translation": "World English Bible"
    }}
]

Instructions for Use:

    Use the aphorisms and associated content in educational materials, sermons, or as part of a motivational series.
    Ensure each element is clearly connected to the themes of the bible chapter reference to maintain scriptural integrity and relevance.

This structured prompt provides a comprehensive guideline for generating content that ties biblical teachings to practical, everyday applications, making ancient wisdom accessible and relevant to a modern audience.

Here is the reference to Job {chapter}:
{script}
"""

print(prompt)
