# Developer Week 2021 - Wer rusted, der rostet nicht

[Developer Week 2021](https://www.developer-week.de/) in Nürnberg.

[Link zur Präsentation](http://wer-rusted-der-rostet-nicht.surge.sh)

## Abstract

Laut einer Stack Overflow-Umfrage aus dem Jahr 2019 ist rust zum vierten mal in Folge zur beliebtesten Programmiersprache unter Softwareentwicklern gewählt worden. Warum ist das so? Was ist so besonders an rust und warum lohnt es sich, mit der Sprache vertraut zu werden, obwohl derzeit noch kaum eine Firma nach rust-Entwicklern sucht?

In meinem Vortrag werde ich einen Einblick in die Programmiersprache rust geben. Ich werde mittels einer Live-Coding Session auf die grundlegenden Sprachkonzepte und Tools eingehen und zeigen, warum es Sinn macht sich die most loved Programmiersprache 2019 näher anzusehen.

## Presentation Tool

Use [mdx deck](https://github.com/jxnblk/mdx-deck)

## Getting Started

``` sh
npm i -D mdx-deck
```

``` sh
cd ./presentation
npm start
```

Hit `alt+P` to start the presenter mode.

## Export Web

``` sh
npm run-script build
```

## Export PDF

See [mdx export docs](https://github.com/jxnblk/mdx-deck/blob/master/docs/exporting.md)

``` sh
npm start
npx website-pdf http://localhost:8000/print -o deck.pdf
```
