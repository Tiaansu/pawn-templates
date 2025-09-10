# Template Engine for Pawn

```pawn
new Template:templateBanned = CreateTemplate(
    "Your account {{ name }} has been banned!\n\n\
    Reason: {{ reason }}\n\
    Duration: {{ days }} days\n\
    If you disagree, please file an appeal at: {{ forum }}");

// ...

new dest[256];
RenderTemplate(templateBanned, dest, sizeof dest,
    PAIR_STR("name", "playerName"),
    PAIR_STR("reason", "Hello World"),
    PAIR_INT("days", 30),
    PAIR_STR("forum", "https://forum.website.com")
);
```

> [!NOTE]   
> Special credits to Southclaws for making [pawn-templates](https://github.com/Southclaws/pawn-templates).