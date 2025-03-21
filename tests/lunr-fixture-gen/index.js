let lunr = require('lunr');
require("lunr-languages/lunr.stemmer.support.js")(lunr);

const fs = require('fs');

for (let file of fs.readdirSync("../data")) {
    if (file.endsWith(".in.txt")) {
        let code = file.substring(0, 2);
        let inp = fs.readFileSync(`../data/${code}.in.txt`);
        let outf = fs.openSync(`../data/${code}.out.txt`, 'w');

        let pipeline;
        let tokenizer;
        if (code === "en") {
            pipeline = new lunr.Pipeline();
            pipeline.add(lunr.trimmer);
            pipeline.add(lunr.stopWordFilter);
            pipeline.add(lunr.stemmer);
            tokenizer = lunr.tokenizer;
        } else {
            if (code === 'ja') {
                let TinySegmenter = require('lunr-languages/tinyseg');
                TinySegmenter(lunr);
            }
            require(`lunr-languages/lunr.${code}.js`)(lunr);

            // Locale functions can do arbitrary things to load themselves (like replace the tokenizer), so we need to
            // run them as they expect (as a lunr pipeline plugin) and use the final configuration after they're called.
            lunr(function () {
                this.use(lunr[code]);
                pipeline = this.pipeline;
                tokenizer = this.tokenizer;
            });
        }
        let tokens = tokenizer(inp);
        tokens = pipeline.run(tokens);

        for (let tok of tokens) {
            tok = tok.toString();
            if (tok)
                fs.writeSync(outf, tok + '\n');
        }
        fs.closeSync(outf);
    }
}
