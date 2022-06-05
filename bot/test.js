const fetch = require('node-fetch');
const split = require('lodash/split');
const reverse = require('lodash/reverse');

const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));

const request = async () => split(await (await fetch(`https://logs6.mcskill.net/Magiccraft_Contigo_public_logs/${("0" + new Date().getDate()).slice(-2)}-${("0" + (new Date().getMonth() + 1)).slice(-2)}-${new Date().getFullYear()}.txt`)).text(), /\r\n|\r|\n/); // TODO: сделать красивее это уродство...

(async () => {
	const logs1 = await request();

	let lastLenght = logs1.length;
	while (true) {
		let logsArr = [];
		const logs2 = await request();

		for (let i = 0; i < logs2.length - lastLenght; i++) {
			let count = logs2.length - 2 - i;
			logsArr.push(logs2[count]);
			count++;
		}
		reverse(logsArr);
		lastLenght = logs2.length;
		console.log(logsArr.join(','));
		await sleep(10000);
	}
})();
