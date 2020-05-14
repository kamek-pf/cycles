const { thingA } = require('./submoduleA');
const { thingB } = require('./submoduleB');

exports.commonThing = "hehe";
exports.thingA = thingA;
exports.thingB = thingB;
