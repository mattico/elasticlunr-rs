let lunr = require('lunr');
require("lunr-languages/lunr.stemmer.support.js")(lunr);

const fs = require('fs');

for (let file of fs.readdirSync("../data")) {
    if (file.endsWith(".in.txt")) {
        let code = file.substring(0, 2);
        let inp = fs.readFileSync(`../data/${code}.in.txt`);
        let outf = fs.openSync(`../data/${code}.out.txt`, 'w');

        let pipeline = new lunr.Pipeline;
        if (code !== "en") {
            if (code === 'ja') {
                let TinySegmenter = require('lunr-languages/tinyseg');
                TinySegmenter(lunr);
            }
            require(`lunr-languages/lunr.${code}.js`)(lunr);

            let lang = lunr[code];
            if (lang.trimmer) pipeline.add(lang.trimmer);
            if (lang.stopWordFilter) pipeline.add(lang.stopWordFilter);
            if (lang.stemmer) pipeline.add(lang.stemmer);
        } else {
            pipeline.add(lunr.trimmer);
            pipeline.add(lunr.stopWordFilter);
            pipeline.add(lunr.stemmer);
        }
        let tokens = lunr.tokenizer(inp);
        tokens = pipeline.run(tokens);

        for (let tok of tokens) {
            tok = tok.toString();
            if (tok && tok.length > 0)
                fs.writeSync(outf, tok + '\n');
        }
        fs.closeSync(outf);
    }
}
