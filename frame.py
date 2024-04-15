import pandas as pd
import json
import requests
import os

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
    

def send_chat_completion(api_key, prompt):
    """
    Sends a chat completion request to OpenAI's GPT-3.5 Turbo API.

    Parameters:
    api_key (str): Your OpenAI API key.
    prompt (str): The chat prompt to send to the model.

    Returns:
    dict: The response from the API.
    """
    url = "https://api.openai.com/v1/chat/completions"
    headers = {
        "Authorization": f"Bearer {api_key}",
        "Content-Type": "application/json",
    }
    data = {
        "model": "gpt-3.5-turbo-0125",
        "messages": [
            {"role": "user", "content": prompt}
        ],
        "response_format": { "type": "json_object" },
        "max_tokens": 4096
    }

    response = requests.post(url, json=data, headers=headers)
    response.raise_for_status()  # Raises an HTTPError for bad responses
    return response.json()

def update_aphorisms_file(api_response, filepath="data/aphorisms.json"):
    """
    Updates a JSON file with the new API response data.

    Parameters:
    api_response (dict): Response data from the API.
    filepath (str): Path to the JSON file where data is saved.
    """
    # Read existing data
    try:
        with open(filepath, "r", encoding='utf-8') as file:
            data = json.load(file)
    except FileNotFoundError:
        data = []

    # Append new data
    if isinstance(api_response, dict):
        if 'aphorisms' in api_response:
            data.extend(api_response["aphorisms"])
        else:
            data.append(api_response)
    elif isinstance(api_response, list):
        data.extend(api_response)

    # Write updated data back to file
    with open(filepath, "w") as file:
        json.dump(data, file, indent=4)


chapters = range(15, 42)
for chapter in chapters:

    df = pd.read_csv("similarity_report.csv", header=0, encoding='utf-8').sort_values(by='Verse Key')
    row_count = df.shape[0]
    df.drop_duplicates(inplace=True)
    new_row_count = df.shape[0]
    if row_count != new_row_count:  # fix up duplicates due to the hastily way the extraction process was written (not removing files yet so we double load them)
        df.to_csv("similarity_report.csv", index=False)
        print("Dropped found duplicates.")


    # Provide me some aphorisms that are abstracted to real life concepts and adages from this bible chapter 
    # In addition to explaining the bible verse, come up with a real life story too
    # When you came up with the aphorisms were you able to identify a reference as far as chapter, verse range for each?
    df2 = df[(df['Translation 1'] == 'World English Bible') & (df['Chapter'] == chapter)][['Text 1']]
    df2['Text 1'] = df2['Text 1'].apply(lambda x: x.replace('“', '"')).apply(lambda x: x.replace('”', '"')).apply(lambda x: x.replace('’', "'")).apply(lambda x: x.replace('‘', "'"))
    script = '\n'.join(df2['Text 1'].tolist())

    prompt = f"""
    Objective:
    Create a set of six aphorisms, explanations, real-life stories from a specific chapter in the Bible. Each element should be tied to the content and themes of the selected chapter, reflecting both spiritual insights and practical applications.
    You must return the response in JSON format with no spaces or newlines after the objects.

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
            Description: For each aphorism, identify specific verse references from the bible chapter verse that support or relate to the aphorism and its explanation.
            Output Requirements: Verse references for each aphorism in the format "Book Chapter:startVerse-endVerse"

        Identify Verse Text:
            Description: For each verse reference, identify the original text from the bible chapter using the "text" field.
            Output Requirements: Verse text for each reference.
            
        Specify the "translation" field as World English Bible

    Expected Format:

        The output should be a list of dictionaries. 
        Each dictionary represents an aphorism along with its explanation, a real-life story,  the specific verse references, the full text of the referenced verses, the translation as 'World English Bible', 

    Example schema for a single object:
    {{
        "aphorism": "In the depths of despair, the soul longs to be heard.",
        "explanation": "Job expresses his deep anguish and weariness of life, longing to express his grievances and find solace in being heard, highlighting the human need for empathy and understanding during times of despair.",
        "story": "A troubled individual seeks out a compassionate friend to share their burdens, finding comfort and relief in the act of expressing their innermost struggles and feelings.",
        "reference": "Job 10:1-2",
        "text": "My soul is weary of my life.\nI will give free course to my complaint.\nI will speak in the bitterness of my soul.",
        "translation": "World English Bible"
    }}

    Instructions for Use:

        Use the aphorisms and associated content in educational materials, sermons, or as part of a motivational series.
        Ensure each element is clearly connected to the themes of the bible chapter reference to maintain scriptural integrity and relevance.

    This structured prompt provides a comprehensive guideline for generating content that ties biblical teachings to practical, everyday applications, making ancient wisdom accessible and relevant to a modern audience.

    Here is the reference to Job {chapter}:
    {script}
    """

    # Usage example:
    api_key = os.getenv("OPENAI_API_KEY")  # More secure way to get environment variable

    attempt_count = 0
    max_attempts = 3
    while attempt_count < max_attempts:
        try:
            response = send_chat_completion(api_key, prompt)
            print(f"API Response (Attempt {attempt_count + 1}):", response)
            response_content = response['choices'][0]['message']['content']
            response_content_json = json.loads(response_content)

            # Update the file with response content
            update_aphorisms_file(response_content_json)

            # If the response is a list, break the loop; if it's a dict, continue to the next attempt
            if isinstance(response_content_json, list):
                break
            attempt_count += 1

        except json.JSONDecodeError as json_err:
            print("JSON decoding failed:", json_err)
        except requests.exceptions.RequestException as req_err:
            print("Request failed:", req_err)
        except Exception as e:
            print("An unexpected error occurred:", e)