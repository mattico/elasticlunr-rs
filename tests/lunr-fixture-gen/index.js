const fs = require('fs');
const elasticlunr = require('elasticlunr');
require("lunr-languages/lunr.stemmer.support.js")(elasticlunr);

for (let file of fs.readdirSync("../data")) {
    if (file.endsWith(".in.txt")) {
        let lang = file.substring(0, 2);
        let index = initializeIndex(lang);
        generateFixtures(lang, index);
    }
}

function initializeIndex(lang) {
    if (lang === "en") {
        return elasticlunr(function () {
            this.addField('body');
        });
    } else {
        if (lang === 'ja') {
            let TinySegmenter = require('lunr-languages/tinyseg');
            TinySegmenter(elasticlunr);
        }
        require(`lunr-languages/lunr.${lang}.js`)(elasticlunr);

        // Locale functions can do arbitrary things to load themselves (like replace the tokenizer), so we need to
        // run them as they expect (as a lunr pipeline plugin) and use the final configuration after they're called.
        return elasticlunr(function () {
            this.use(lunr[lang]);
            this.addField('body');
        });
    }
}

function generateFixtures(lang, index) {
    let input = fs.readFileSync(`../data/${lang}.in.txt`);
    let textOutput = fs.openSync(`../data/${lang}.out.txt`, 'w');
    let indexOutput = fs.openSync(`../data/${lang}.index.json`, 'w');

    let tokens = tokenizer(input);
    tokens = pipeline.run(tokens);

    for (let tok of tokens) {
        tok = tok.toString();
        if (tok)
            fs.writeSync(textOutput, tok + '\n');
    }
    fs.closeSync(textOutput);
}
