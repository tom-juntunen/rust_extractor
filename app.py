from flask import Flask, render_template
import pandas as pd
import json

app = Flask(__name__)


def load_aphorisms():
    with open('data/aphorisms.json', 'r') as file:
        aphorisms = json.load(file)
    return aphorisms

def load_similarity_report():
    df = pd.read_csv('similarity_report.csv')
    
    # Ensure that 'Chapter' and 'Verse' are integers to remove any leading zeros.
    df['Chapter'] = df['Chapter'].astype(int)
    df['Verse'] = df['Verse'].astype(int)
    
    # Group by 'Chapter' and 'Verse', then calculate the mean 'Similarity'.
    grouped = df.groupby(['Chapter', 'Verse'], as_index=False)['Similarity'].mean()
    
    # Create a new column 'row' based on the unique chapters and 'column' based on verses.
    # The following will create a mapping from Chapter and Verse to a row and column index.
    chapter_to_row = {chap: idx for idx, chap in enumerate(grouped['Chapter'].unique(), start=1)}
    verse_to_col = {verse: idx for idx, verse in enumerate(grouped['Verse'].unique(), start=1)}

    grouped['row'] = grouped['Chapter'].map(chapter_to_row)
    grouped['col'] = grouped['Verse'].map(verse_to_col)
    
    # Convert the aggregated DataFrame to a dictionary for passing to the template.
    heatmap_data = grouped[['row', 'col', 'Similarity']].to_dict(orient='records')
    return heatmap_data

@app.route('/')
def index():
    heatmap_data = load_similarity_report()
    aphorisms = load_aphorisms()
    return render_template('heatmap.html', heatmap_data=heatmap_data, aphorisms=aphorisms)

if __name__ == '__main__':
    app.run(debug=True)
