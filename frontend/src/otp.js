import axios from 'axios';
import { STORAGE_KEY, API_BASE_URL, user_input, input_errors, modal } from "./state.js";
import { createApp } from 'https://unpkg.com/petite-vue?module';

function delete_from_local_storage() {
  localStorage.removeItem(STORAGE_KEY);
}

function focusNextInput(el, prevId, nextId) {
  if (el.value.length === 0) {
    if (prevId) {
      document.getElementById(prevId).focus();
    }
  } else {
    if (nextId) {
      document.getElementById(nextId).focus();
    }
  }
}

export const otp_verification = {
  modal,
  timer: 30,
  user_input,
  input_errors,

  isLoading: false,
  resendDisabled: true,

  async showView() {
    const html = await fetch('views/otp-verification.html').then(res => res.text());
    document.getElementById('otp').innerHTML = html;

    createApp(otp_verification).mount();

    this.startCountdown();
  },

  startCountdown() {
    this.timer = 30;
    this.resendDisabled = true;
    const interval = setInterval(() => {
      if (this.timer > 1) {
        this.timer--;
      } else {
        this.resendDisabled = false;
        clearInterval(interval);
      }
    }, 1000);
  },

  go_back() {
    document.getElementById('otp').innerHTML = '';
  },

  submitOTP() {
    const inputs = document.querySelectorAll('[data-focus-input-init]');
    const otp = Array.from(inputs).map(input => input.value).join('');

    if (otp.length < 6) {
      this.input_errors.otp = 'Please enter a valid OTP.';
      return;
    }

    this.isLoading = true;

    axios.post(`${API_BASE_URL}/verify-otp`, {
      email: this.user_input.email,
      otp: otp
    }, {
      headers: {
        'Content-Type': 'application/json'
      }
    })
    .then(response => {
      this.isLoading = false;
      this.input_errors = { };

      delete_from_local_storage();
      this.modal.openSuccessModal();
    })
    .catch(error => {
      this.isLoading = false;

      if (error.response) {
        if (error.response.status === 400) {
          this.input_errors = error.response.data.errors;
        } else if (error.response.status === 429) {
          // this.
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
  },

  resend_otp() {
    axios.post(`${API_BASE_URL}/send-future-letter`, {
      name: this.user_input.name,
      email: this.user_input.email,
      message: this.user_input.message,
      delivery_date: new Date(this.user_input.delivery_date).toISOString(),
    }, {
      headers: {
        'Content-Type': 'application/json'
      }
    })
    .then(response => {
      // show that the otp has been sent 
      console.log('OTP resent');
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

    this.startCountdown();
  },

  handleKeyup(e) {
    const el = e.target;
    const prevId = el.getAttribute('data-focus-input-prev');
    const nextId = el.getAttribute('data-focus-input-next');
    focusNextInput(el, prevId, nextId);

    const inputs = document.querySelectorAll('[data-focus-input-init]');
    const allFilled = Array.from(inputs).every(input => input.value.trim() !== '');

    if (allFilled) {
      this.submitOTP();
    }
  },
  
  handlePaste(e) {
    e.preventDefault();

    const pasteData = (e.clipboardData || window.clipboardData).getData('text');
    const digits = pasteData.replace(/\D/g, '');
    const inputs = document.querySelectorAll('[data-focus-input-init]');

    inputs.forEach((input, index) => {
      if (digits[index]) {
        input.value = digits[index];
        const nextId = input.getAttribute('data-focus-input-next');
        if (nextId) {
          document.getElementById(nextId).focus();
        }
      }
    });
  },
};
