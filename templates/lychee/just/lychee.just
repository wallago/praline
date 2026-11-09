# {if:just}
# Check every link in the Markdown files
[group('quality')]
links:
    lychee --no-progress '**/*.md'
# {endif:just}
