var lunr = require('lunr');
require("lunr-languages/lunr.stemmer.support.js")(lunr);
const fs = require('fs');

for (let file of fs.readdirSync("../data")) {
    if (file.endsWith(".in.txt")) {
        let code = file.substring(0, 2);
        let inp = fs.readFileSync(`../data/${code}.in.txt`);
        let outf = fs.openSync(`../data/${code}.out.txt`, 'w');

        var pipeline = new lunr.Pipeline;
        if (code !== "en")
        {
            require(`lunr-languages/lunr.${code}.js`)(lunr);

            pipeline.add(lunr[code].trimmer);
            pipeline.add(lunr[code].stopWordFilter);
            pipeline.add(lunr[code].stemmer);
        } else {
            pipeline.add(lunr.trimmer);
            pipeline.add(lunr.stopWordFilter);
            pipeline.add(lunr.stemmer);
        }
        var tokens = lunr.tokenizer(inp);
        tokens = pipeline.run(tokens);

        for (var tok of tokens) {
            tok = tok.toString();
            if (tok && tok.length > 0)
                fs.writeSync(outf, tok + '\n');
        }
        fs.closeSync(outf);
    }
}
