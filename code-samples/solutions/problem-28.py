def starts_with_any(text, prefixes):
    return any(text.startswith(prefix) for prefix in prefixes)
