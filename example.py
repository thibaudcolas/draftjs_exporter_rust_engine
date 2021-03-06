# -*- coding: utf-8 -*-
import codecs
import json

from draftjs_exporter.constants import BLOCK_TYPES, ENTITY_TYPES, INLINE_STYLES
from draftjs_exporter.defaults import BLOCK_MAP, STYLE_MAP
from draftjs_exporter.dom import DOM
from draftjs_exporter.html import HTML
from draftjs_exporter.types import Element, Props

def image(props: Props) -> Element:
    return DOM.create_element('img', {
        'src': props.get('src'),
        'width': props.get('width'),
        'height': props.get('height'),
        'alt': props.get('alt'),
    })


def link(props: Props) -> Element:
    return DOM.create_element('a', {
        'href': props['url']
    }, props['children'])

content_state = {
    "entityMap": {
        "0": {
            "type": "LINK",
            "mutability": "MUTABLE",
            "data": {
                "url": "https://github.com/facebook/draft-js"
            }
        },
    },
    "blocks": [
        {
        "key": "b0ei9",
        "text": "draftjs_exporter is an HTML exporter for Draft.js content",
        "type": "header-two",
        "depth": 0,
        "inlineStyleRanges": [],
        "entityRanges": [
            {
            "offset": 41,
            "length": 8,
            "key": 0
            }
        ],
        "data": {}
        },
    ]
}

# Demo content from https://github.com/springload/draftjs_exporter/blob/master/example.py.
# with open('docs/example.json') as example:
#     content_state = json.load(example)

if __name__ == '__main__':
    exporter = HTML({
        'block_map': BLOCK_MAP,
        'style_map': STYLE_MAP,
        'entity_decorators': {
            # Map entities to components so they can be rendered with their data.
            ENTITY_TYPES.IMAGE: image,
            ENTITY_TYPES.LINK: link,
            # Lambdas work too.
            ENTITY_TYPES.HORIZONTAL_RULE: lambda props: DOM.create_element('hr'),
            # Discard those entities.
            ENTITY_TYPES.EMBED: None,
        },
        'engine': 'draftjs_exporter_rust_engine.engine.DOMString',
    })

    markup = exporter.render(content_state)

    print(markup)

    # Output to a Markdown file to showcase the output in GitHub (and see changes in git).
    with codecs.open('docs/example.txt', 'w', 'utf-8') as file:
        file.write(
            """# Example output (generated by [`example.py`](../example.py))
---
{markdown}---
""".format(markdown=markup))

import string_engine

print(string_engine.sum_as_string(4, 3))
