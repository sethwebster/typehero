def to_title_case(s):
    """Convert string to title case"""
    return ' '.join(word.capitalize() for word in s.split())
