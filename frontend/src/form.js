import axios from 'axios';
import { otp_verification } from './otp.js';
import { createApp } from 'https://unpkg.com/petite-vue?module';
import { STORAGE_KEY, API_BASE_URL, user_input, input_errors, modal } from './state.js';

import flatpickr from 'flatpickr';
import 'flatpickr/dist/flatpickr.min.css';

function load_from_local_storage() {
  const saved = localStorage.getItem(STORAGE_KEY);
  if (saved) {
    try {
      Object.assign(user_input, JSON.parse(saved));
    } catch {}
  }
}

export const form = {
  modal,
  user_input,
  input_errors,

  isLoading: false,
  formatted_date: '',

  save_to_local_storage() {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(user_input));
  },

  async showView() {
    load_from_local_storage();

    const html = await fetch('views/form.html').then(res => res.text());
    document.getElementById('app').innerHTML = html;

    createApp(form).mount();
    this.initDateTimeInput();
    this.format_date();
  },

  initDateTimeInput() {
    flatpickr("#datetime", {       
      minDate: new Date().fp_incr(1),
      dateFormat: "Y-m-d",
    });
  },

  format_date() {
    if (this.user_input.delivery_date) {
      const date = new Date(this.user_input.delivery_date);
      this.formatted_date = date.toLocaleDateString("en-US", {
        weekday: "long",
        year: "numeric",
        month: "long",
        day: "numeric"
      });
    } else {
      this.formatted_date = '';
    }
  },

  submit() {
    if (!user_input.delivery_date) {
      input_errors.delivery_date = 'Delivery date is required.';
      return;
    }

    this.isLoading = true;

    console.log(new Date().toISOString);

    axios.post(`${API_BASE_URL}/send-future-letter`, {
      name: this.user_input.name,
      email: this.user_input.email,
      message: this.user_input.message,
      delivery_date: new Date(this.user_input.delivery_date).toISOString(),
    },{
        headers: {
          'Content-Type': 'application/json'
        }
    })
    .then(response => {
      this.isLoading = false;
      otp_verification.showView();
    })
    .catch(error => {
      this.isLoading = false;

      if (error.response) {
        if (error.response.status === 400) {
          this.input_errors = error.response.data.errors;
        } else if (error.response.status === 500) {
          this.modal.openErrorModal();
        }
      } else if (error.request) {
        // No response from server (e.g., offline, server down)
        console.error('No response from server');
        this.modal.openErrorModal("Network error: Please check your internet connection.");
      } else {
        // Something else went wrong in setting up the request
        console.error('Request error', error.message);
        this.modal.openErrorModal("Unexpected error: " + error.message);
      }
    });
  }
};