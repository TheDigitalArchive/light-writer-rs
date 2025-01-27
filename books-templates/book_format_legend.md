# Book Legend

This document provides detailed explanations of each field used in the book creation data structure.

## Metadata

### `authors`
- **Description**: The author(s) of the book.
- **Example**: `"Robert Greene"`

### `production`
- **Description**: Information about the producer or production team.
- **Example**: `"Joost Elffers"`

### `title`
- **Description**: The main title of the book.
- **Example**: `"THE 48 LAWS OF POWER"`

### `subtitle`
- **Description**: The secondary title or tagline of the book, if any.
- **Example**: `"A Guide to Mastery"`

### `isbn`
- **Description**: International Standard Book Number for the book.
- **Example**: `"9780140280197"`

### `publisher`
- **Description**: The publishing house responsible for releasing the book.
- **Example**: `"Penguin Books"`

### `publication_date`
- **Description**: The date when the book was published.
- **Example**: `"1998-09-01"`

### `language`
- **Description**: The language in which the book is written.
- **Example**: `"English"`

### `genres`
- **Description**: The genre(s) of the book.
- **Example**: `["Self-Help", "Non-Fiction"]`

### `tags`
- **Description**: Additional tags or keywords for categorization.
- **Example**: `["Leadership", "Strategy"]`

### `edition`
- **Description**: The edition of the book, if applicable.
- **Example**: `"2nd Edition"`

## Content Structure

### `description`
- **Description**: A summary or blurb about the book.
- **Example**: `"An insightful guide on power dynamics and strategies for success."`

### `table_of_contents`
- **Description**: A list of chapters or sections in the book.
- **Example**: `["Introduction", "Law 1: Never Outshine the Master"]`

### `page_index`
- **Description**: The current page or starting index for content organization.
- **Example**: `0`

### `page_count`
- **Description**: Total number of pages in the book.
- **Example**: `450`

### `content`
- **Description**: The primary text or body of the book.
- **Example**: `"Chapter 1: The journey begins..."`

### `notes`
- **Description**: General notes about the book or its content.
- **Example**: `"Focuses heavily on historical examples."`

### `add_notes`
- **Description**: Additional notes or annotations.
- **Example**: `"Expand on Law 4 for clarity."`

### `quotes`
- **Description**: Notable quotes from the book.
- **Example**: `["Power is not what you have but what others think you have."]`

### `references`
- **Description**: A list of references or citations included in the book.
- **Example**: `["Machiavelli, The Prince"]`

### `keywords`
- **Description**: Key terms or phrases relevant to the book's content.
- **Example**: `["Power", "Strategy", "Leadership"]`

## Progress Tracking

### `reading_progress`
- **Description**: Tracks the reader's progress in the book.
- **Structure**:
  ```json
  {
      "current_page": 0,
      "percentage_completed": 0
  }
  ```
- **Example**:
  ```json
  {
      "current_page": 125,
      "percentage_completed": 55.5
  }
  ```

## Engagement

### `reviews`
- **Description**: Reader reviews or comments about the book.
- **Example**: `["A thought-provoking read."]`

### `rating`
- **Description**: An overall rating for the book, typically out of 5.
- **Example**: `4.5`

## Publishing and People

### `illustrator`
- **Description**: The illustrator of the book, if any.
- **Example**: `"Jane Doe"`

### `editor`
- **Description**: The editor responsible for the content.
- **Example**: `"John Smith"`

### `translator`
- **Description**: The person who translated the book, if applicable.
- **Example**: `"Anna Lee"`

### `dedication`
- **Description**: A dedication section in the book.
- **Example**: `"To all who dare to dream."`

### `acknowledgments`
- **Description**: A section to thank contributors or supporters.
- **Example**: `"Special thanks to my mentor."`

## Additional Context

### `introduction`
- **Description**: An introductory section that sets the stage for the book.
- **Example**: `"This book explores timeless laws of power..."`

### `preface`
- **Description**: A preliminary statement by the author about the book's purpose.
- **Example**: `"I wrote this book to demystify power dynamics."`

### `appendices`
- **Description**: Additional sections with supplementary material.
- **Example**: `["Appendix A: Historical Context"]`

### `index_terms`
- **Description**: Terms used in the index for reference.
- **Example**: `["Power", "Strategy"]`

### `related_books`
- **Description**: Books similar or related to this one.
- **Example**: `["The Art of War", "The Prince"]`

### `resources`
- **Description**: External resources for further exploration.
- **Example**: `["www.powerdynamics.com"]`

### `format`
- **Description**: The format of the book.
- **Example**: `"Hardcover"`
