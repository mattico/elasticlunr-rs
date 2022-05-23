/*!
 * Simple Arabic stemmer based on lunr.ar.js from https://github.com/MihaiValentin/lunr-languages
 *
 * Copyright 2018, Dalia Al-Shahrabi
 * http://www.mozilla.org/MPL/
 */
/*!
 */

/**
 * export the module via AMD, CommonJS or as a browser global
 * Export code from https://github.com/umdjs/umd/blob/master/returnExports.js
 */
;
(function (root, factory) {
  if (typeof define === 'function' && define.amd) {
    // AMD. Register as an anonymous module.
    define(factory)
  } else if (typeof exports === 'object') {
    /**
     * Node. Does not work with strict CommonJS, but
     * only CommonJS-like environments that support module.exports,
     * like Node.
     */
    module.exports = factory()
  } else {
    // Browser globals (root is window)
    factory()(root.lunr);
  }
}(this, function () {
  /**
   * Just return a value to define the module export.
   * This example returns an object, but the module
   * can return a function as the exported value.
   */
  return function (lunr) {
    /* throw error if lunr is not yet included */
    if ('undefined' === typeof lunr) {
      throw new Error('Lunr is not present. Please include / require Lunr before this script.');
    }

    /* register specific locale function */
    lunr.ar = function () {
      this.pipeline.reset();
      this.pipeline.add(
        lunr.ar.stemmer
      );

      // for lunr version 2
      // this is necessary so that every searched word is also stemmed before
      // in lunr <= 1 this is not needed, as it is done using the normal pipeline
      if (this.searchPipeline) {
        this.searchPipeline.reset();
        this.searchPipeline.add(lunr.ar.stemmer)
      }
    };

    /* lunr stemmer function */
    lunr.ar.stemmer = (function () {

      /* remove elongating character */
      self.removeElongating = function (word) {
        return word.replace(/[\u0640]/gi, '');
      }

      self.removeDiacritics = function (word) {
        return word.replace(/[\u064b-\u065b]/gi, '');
      }

      /*Replace all variations of alef (آأإٱى) to a plain alef (ا)*/
      self.cleanAlef = function (word) {
        return word.replace(/[\u0622\u0623\u0625\u0671\u0649]/gi, "\u0627");
      }

      self.execArray = [
        'removeElongating',
        'removeDiacritics',
        'cleanAlef'
      ];

      self.stem = function (word) {
        var counter = 0;
        while (counter < self.execArray.length) {
          word = self[self.execArray[counter]](word);
          counter++;
        }
        return word;
      }

      return function (word) {
        return self.stem(word);
      }
    })();

    lunr.Pipeline.registerFunction(lunr.ar.stemmer, 'stemmer-ar');
  };
}))