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

# Provide me some aphorisms that are abstracted to real life concepts and adages from this bible chapter 
# In addition to explaining the bible verse, come up with a real life story too
# When you came up with the aphorisms were you able to identify a reference as far as chapter, verse range for each?
df2 = df[(df['Translation 1'] == 'World English Bible') & (df['Chapter'] == 2)][['Text 1']]
print('\n'.join(df2['Text 1'].tolist()))


prompt = """
Objective:
Create a set of aphorisms, explanations, real-life stories, and verse references from a specific chapter in the Bible. Each element should be tied to the content and themes of the selected chapter, reflecting both spiritual insights and practical applications.

Bible Chapter:

    Book Name: Job
    Chapter Number: 2

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

    The output should be a list of dictionaries. Each dictionary represents an aphorism along with its explanation, a real-life story, and the specific verse references from the bible chapter verse. The dictionaries should have the following keys: aphorism, explanation, story, reference.

Example Output:

json

[
    {
        "aphorism": "Fortune is a wheel; it turns endlessly, elevating and humbling.",
        "explanation": "This reflects the unpredictable nature of life's circumstances, as seen in Job’s sudden loss despite his righteousness.",
        "story": "Consider a successful entrepreneur who faces a sudden downturn due to market changes, mirroring Job’s trials and testing their resilience and moral fiber.",
        "reference": "Job 1:1-3, 1:13-19"
    },
    {
        "aphorism": "Virtue is tested in the fires of adversity.",
        "explanation": "True character is revealed through challenges, as Job maintains his integrity despite his losses.",
        "story": "A community leader continues to advocate for the needy despite facing personal tragedies, showing deep commitment to her principles.",
        "reference": "Job 1:20-22"
    }
]

Instructions for Use:

    Use the aphorisms and associated content in educational materials, sermons, or as part of a motivational series.
    Ensure each element is clearly connected to the themes of the bible chapter reference to maintain scriptural integrity and relevance.

This structured prompt provides a comprehensive guideline for generating content that ties biblical teachings to practical, everyday applications, making ancient wisdom accessible and relevant to a modern audience.

Here is the reference to Job 2:


Again, on the day when the God’s sons came to present themselves before Yahweh, Satan came also among them to present himself before Yahweh.

Yahweh said to Satan, “Where have you come from?”
Satan answered Yahweh, and said, “From going back and forth in the earth, and from walking up and down in it.”

Yahweh said to Satan, “Have you considered my servant Job? For there is no one like him in the earth, a blameless and an upright man, one who fears God, and turns away from evil. He still maintains his integrity, although you incited me against him, to ruin him without cause.”

Satan answered Yahweh, and said, “Skin for skin. Yes, all that a man has he will give for his life.

But stretch out your hand now, and touch his bone and his flesh, and he will renounce you to your face.”

Yahweh said to Satan, “Behold, he is in your hand. Only spare his life.”

So Satan went out from the presence of Yahweh, and struck Job with painful sores from the sole of his foot to his head.

He took for himself a potsherd to scrape himself with, and he sat among the ashes.

Then his wife said to him, “Do you still maintain your integrity? Renounce God, and die.”

But he said to her, “You speak as one of the foolish women would speak. What? Shall we receive good at the hand of God, and shall we not receive evil?”
In all this Job didn’t sin with his lips.

Now when Job’s three friends heard of all this evil that had come on him, they each came from his own place: Eliphaz the Temanite, Bildad the Shuhite, and Zophar the Naamathite; and they made an appointment together to come to sympathize with him and to comfort him.      

When they lifted up their eyes from a distance, and didn’t recognize him, they raised their voices, and wept; and they each tore his robe, and sprinkled dust on their heads toward the sky.

So they sat down with him on the ground seven days and seven nights, and no one spoke a word to him, for they saw that his grief was very great.
"""


