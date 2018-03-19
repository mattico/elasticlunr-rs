use pipeline::Pipeline;

pub fn make_pipeline() -> Pipeline {
    Pipeline {
        queue: vec![
            ("trimmer-fr".into(), trimmer),
            ("stopWordFilter-fr".into(), stop_word_filter),
            ("stemmer-fr".into(), stemmer),
        ],
    }
}

make_trimmer!(
    "A-Za-z\\xAA\\xBA\\xC0-\\xD6\\xD8-\\xF6\\xF8-\\u02B8\\u02E0-\\u02E4\\u1D00-\\u1D25\
     \\u1D2C-\\u1D5C\\u1D62-\\u1D65\\u1D6B-\\u1D77\\u1D79-\\u1DBE\\u1E00-\\u1EFF\\u2071\\u207F\
     \\u2090-\\u209C\\u212A\\u212B\\u2132\\u214E\\u2160-\\u2188\\u2C60-\\u2C7F\\uA722-\\uA787\
     \\uA78B-\\uA7AD\\uA7B0-\\uA7B7\\uA7F7-\\uA7FF\\uAB30-\\uAB5A\\uAB5C-\\uAB64\\uFB00-\\uFB06\
     \\uFF21-\\uFF3A\\uFF41-\\uFF5A"
);

make_stop_word_filter!([
    "", "ai", "aie", "aient", "aies", "ait", "as", "au", "aura", "aurai", "auraient", "aurais",
    "aurait", "auras", "aurez", "auriez", "aurions", "aurons", "auront", "aux", "avaient", "avais",
    "avait", "avec", "avez", "aviez", "avions", "avons", "ayant", "ayez", "ayons", "c", "ce",
    "ceci", "celà", "ces", "cet", "cette", "d", "dans", "de", "des", "du", "elle", "en", "es",
    "est", "et", "eu", "eue", "eues", "eurent", "eus", "eusse", "eussent", "eusses", "eussiez",
    "eussions", "eut", "eux", "eûmes", "eût", "eûtes", "furent", "fus", "fusse", "fussent",
    "fusses", "fussiez", "fussions", "fut", "fûmes", "fût", "fûtes", "ici", "il", "ils", "j",
    "je", "l", "la", "le", "les", "leur", "leurs", "lui", "m", "ma", "mais", "me", "mes", "moi",
    "mon", "même", "n", "ne", "nos", "notre", "nous", "on", "ont", "ou", "par", "pas", "pour",
    "qu", "que", "quel", "quelle", "quelles", "quels", "qui", "s", "sa", "sans", "se", "sera",
    "serai", "seraient", "serais", "serait", "seras", "serez", "seriez", "serions", "serons",
    "seront", "ses", "soi", "soient", "sois", "soit", "sommes", "son", "sont", "soyez", "soyons",
    "suis", "sur", "t", "ta", "te", "tes", "toi", "ton", "tu", "un", "une", "vos", "votre", "vous",
    "y", "à", "étaient", "étais", "était", "étant", "étiez", "étions", "été", "étée",
    "étées", "étés", "êtes",
]);

make_stemmer!(Algorithm::French);

#[cfg(feature = "bench")]
mod benches {
    extern crate test;
    use super::*;

    // # Results
    // HashSet:  175,669 ns/iter (+/- 15,652)
    // BTreeSet: 210,169 ns/iter (+/- 29,430)
    // PHF:      159,961 ns/iter (+/- 16,492)

    #[bench]
    fn bench_stop_word(b: &mut test::Bencher) {
        let data = ::pipeline::tokenize(TEXT);
        let data: Vec<_> = data.into_iter().filter_map(|t| trimmer(t)).collect();
        b.iter(|| {
            let d = data.clone();
            for word in d {
                test::black_box(stop_word_filter(word));
            }
        });

        const TEXT: &str = "
QUELQUES heures après la mort de l’Impératrice Catherine, son fils,
l’Empereur Paul, ordonna au comte Rostoptchine de mettre les scellés sur les
papiers de l’Impératrice. Il était lui-même présent à la mise en ordre de
ces papiers. On y trouva la célèbre lettre d’Alexis Orloff,[A]—par
laquelle, d’un ton cynique et d’une main ivre, il annonçait à
l’Impératrice l’assassinat de son mari, Pierre III,—et un manuscrit
écrit entièrement de la main de Catherine; ce dernier était contenu dans une
enveloppe cachetée, portant cette inscription: Его
Императорскому Высочеству, Цесаревичу и
Великому Князю Павлу Петровичу, любезному
сыну моему. (A Son Altesse Impériale, le Césarewitch, et grand-duc
Paul, mon fils bien aimé.) Sous cette enveloppe se trouvait le manuscrit des
Mémoires que nous publions. Le cahier se termine brusquement vers la fin de
1759. On dit qu’il y avait des notes détachées qui auraient dû servir de
matériaux pour la continuation. Il y a des personnes qui disent que Paul les a
jetées au feu: il n’y a pas de certitude à ce sujet. Paul tenait en grand
secret le manuscrit de sa mère, et ne le confia jamais qu’à son ami
d’enfance, le prince Alexandre Kourakine. Celui-ci en prit une copie. Une
vingtaine d’années après la mort de Paul, Alexandre Tourgeneff et le prince
Michel Worontzoff obtinrent des copies de l’exemplaire de Kourakine.
L’Empereur Nicolas, ayant entendu parler de cela, donna ordre à la police
secrète de s’emparer de toutes les copies. Il y en avait, entr’autres, une
ecrite, à Odessa, par la main du célèbre poète Pouschkine. Effectivement,
les Mémoires de l’Impératrice Catherine II ne circulèrent plus.
L’Empereur Nicolas se fit apporter, par le comte D. Bloudoff, l’original, le
lut, le cacheta avec le grand sceau de l’état, et ordonna de le garder aux
archives impériales, parmi les documents les plus secrets. A ces détails, que
j’extrais d’une notice qui m’a été communiquée, je dois ajouter que la
première personne qui m’en parla, fut le précepteur de l’Empereur actuel,
Constantin Arsenieff. Il me disait, en 1840, qu’il avait obtenu la permission
de lire beaucoup de documents secrets sur les événements qui suivirent la mort
de Pierre I, jusqu’au règne d’Alexandre I. Parmi ces documents, on
l’autorisa à lire les Mémoires de Catherine II. (Il enseignait alors
l’histoire moderne de Russie au grand-duc, Héritier présomptif.) Pendant la
guerre de Crimée on transféra les archives à Moscou. Au mois de mars 1855,
l’Empereur actuel se fit apporter le manuscrit pour le lire. Depuis ce temps
une ou deux copies circulèrent de rechef à Moscou et à Pétersbourg. C’est
sur une de ces copies que nous publions les Mémoires. Quand à
l’authenticité, il n’y a pas le moindre doute. Au reste il suffit de lire
deux ou trois pages du texte pour être convaincu. Nous nous sommes abstenus de
faire des corrections de style, dans tous les cas où nous n’avions pas la
conviction que la copie portait une faute de transcription. Passant aux
mémoires eux-mêmes, qu’avons-nous à dire? Les premières années de
Catherine II—de cette femme-empereur, qui occupa plus d’un quart de siècle
tous les esprits contemporains, depuis Voltaire et Frédéric II jusqu’au Khan
de Crimée et aux chefs des Kirghis—ses jeunes années, racontées par
elle-même!... Qu’y a-t-il, pour l’éditeur, à ajouter à cela? En lisant
ces pages, on la voit venir, on la voit se former telle qu’elle a été plus
tard. Enfant espiègle de quatorze ans, coiffée à la «Moïse,» blonde,
folâtre, fiancée d’un petit idiot—le grand-duc—elle a déjà le mal du
palais d’hiver, la soif de la domination. Un jour, étant «juchée» avec le
grand-duc sur une fenêtre et plaisantant avec lui, elle voit entrer le comte
Lestocq, qui lui dit: «Faites vos paquets,—vous repartirez pour
l’Allemagne.» Le jeune idiot ne semble pas très affecté de cette
séparation possible. «Ce m’était aussi une affaire assez indifférente,»
dit la petite allemande, «mais la couronne de Russie ne me l’était pas,»
ajoute la grande-duchesse. Voilà, en herbe, la Catherine de 1762! Rêver à la
couronne au reste était tout naturel,—dans cette atmosphère de la cour
impériale,—non-seulement pour la fiancée de l’héritier présomptif, mais
pour tout le monde. Le palefrenier Biren, le chanteur Rasoumowsky, le prince
Dolgorouky, le plébéien Menchikoff, l’oligarque Volynski,—tout le monde
voulait avoir un lambeau du manteau impérial. La couronne de Russie
était—après Pierre I—une res nullius. Pierre I, terroriste et réformateur
avant tout, n’avait aucun respect pour la légitimité. Son absolutisme
s’efforçait d’aller même au delà de la tombe. Il se donna le droit de
désigner son successeur, et, au lieu de le faire, il se borna à ordonner
l’assassinat de son propre fils. Après la mort de Pierre I, les grands de
l’état s’assemblent pour aviser. Menchikoff arrête toute délibération,
et proclame impératrice son ancienne maîtresse, veuve d’un brave dragon
suédois, tué sur le champ de bataille, et veuve de Pierre I, auquel Menchikoff
l’avait cédée «par dévouement.» Le règne de Catherine I est court.
Après elle, la couronne continue à passer d’une tête à l’autre, au
hasard: de la ci-devant cabaretière livonienne à un gamin (Pierre II); de ce
gamin, qui meurt de la petite vérole, à la duchesse de Courlande (Anne); de la
duchesse de Courlande à une princesse de Mecklenbourg, mariée à un prince de
Brunswick, qui règne au nom d’un enfant au berceau (Jvan); de l’enfant né
trop tard pour régner, la couronne passe sur la tête d’une fille née trop
tôt—Elisabeth. C’est elle qui représente la légitimité. La tradition
rompue, brisée, le peuple et l’état complètement séparés par la réforme
de Pierre I, les coups d’état, les révolutions de palais étaient alors en
permanence. Rien de stable. En se mettant au lit les habitants de Pétersbourg
ne savaient jamais sous le gouvernement de qui ils se réveilleraient. Aussi
s’intéressait-on fort peu à ces changements, qui ne touchaient au fond que
quelques intrigants allemands devenus ministres russes, quelques grands
seigneurs blanchis dans le parjure et le crime, et le régiment de
Préobrajensky, qui, à l’instar des Prétoriens, disposait de la couronne.
Pour les autres il n’y avait rien de changé. Et quand je dis les autres, je
ne parle que de la noblesse et des employés: car de l’immensité silencieuse
du peuple—du peuple courbé, triste, ahuri, muet—personne ne
s’inquiétait; le peuple restait hors la loi, acceptant passivement
l’épreuve terrible qu’il plaisait au bon Dieu de lui envoyer, et ne se
souciant guère, de son côté, des spectres qui montaient d’un pas chancelant
les marches du trône, glissaient comme des ombres, et disparaissaient en
Sibérie ou dans les casemates. Le peuple, dans tous les cas, était sûr
d’être pillé. Son état social était donc à l’abri de toute chance.
Période étrange! Le trône impérial—comme nous l’avons dit
ailleurs[B]—ressemblait au lit de Cléopatre. Un tas d’oligarques,
d’étrangers, de pandours, de mignons conduisaient nuitamment un inconnu, un
enfant, une allemande; l’élevaient au trône, l’adoraient, et
distribuaient, en son nom, des coups de knout à ceux qui trouvaient à y
redire. A peine l’élu avait-il eu le temps de s’enivrer de toutes les
jouissances d’un pouvoir exorbitant et absurde, et d’envoyer ses ennemis aux
travaux forcés ou à la torture, que la vague suivante apportait déjà un
autre prétendant, et entraînait l’élu d’hier, avec tout son entourage,
dans l’abîme. Les ministres et les généraux du jour s’en allaient le
lendemain, chargés de fer, en Sibérie.";
    }
}
