import 'flowbite';
import './style.css';

import { form } from './form.js';
import axios from 'axios';
import { API_BASE_URL } from './state.js';

async function main() {
  form.showView();

  axios.get(`${API_BASE_URL}/ping`)
    .then(() => console.log("Server is up!"))
    .catch((err) => console.log("Waking up server...", err.message));;
}

main();