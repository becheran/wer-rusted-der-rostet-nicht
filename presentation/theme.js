import React from 'react'
import { Prism } from 'react-syntax-highlighter'
import { vs } from 'react-syntax-highlighter/dist/esm/styles/prism';


const getLanguage = className => {
    const match = /language-(\w*)/.exec(className || 'language-javascript')
    let lang = 'javascript'
    if (match && match.length > 1) {
        lang = match[1]
    }
    return lang
}

const code = props => {
    const language = getLanguage(props.className)
    return <Prism language={language} style={vs} {...props} />
}

export default {
    font: 'Georgia',
    monospace: 'Menlo, monospace',
    fontSizes: [
        '0.75em', '1em', '1.5em', '2em', '3em'
    ],
    colors: {
        text: '#000',
        background: 'white',
        link: '#07c',
        heading: '#000',
        quote: '#000',
        pre: '#f0f',
        preBackground: '#333',
        rustYellow: '#ffc832',
        rustGreen: 'rgb(11, 114, 97)',
        rustRed: 'rgb(167, 33, 69)',
        rustBlue: '#2e2459',
    },
    css: {
        // apply any styles to the root element
        fontWeight: 200
    },
    // custom CSS can be provided to any of the default components:
    heading: {
        fontWeight: 200
    },
    link: {
        textDecoration: 'none',
        '&:hover': {
            textDecoration: 'underline',
        }
    },
    components: {
        code,
    },
    styles: {
        p: {
            fontSize: '1em',
        },
        th: {
            padding: '10px',
            color: 'white',
            bg: 'rustGreen',
        },
        td: {
            border: '1px solid #ddd',
            padding: '8px',
        },
    }
}