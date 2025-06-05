import 'flowbite';
import './style.css';

import { form } from './form.js';
import { otp_verification } from './otp.js';

async function main() {
  form.showView();
  // otp_verification.showView();
}

main();

// const form = {
//   user_input,
//   input_errors,
//   save_to_local_storage,

//   async showView() {
//     const html = await fetch('views/form.html').then(res => res.text());
//     document.getElementById('app').innerHTML = html;

//     createApp(
//       form
//     ).mount();

//     this.initDateTimeInput();
//   },

//   initDateTimeInput() {

//   },

//   submit() {
    // if (!user_input.delivery_date) {
    //   input_errors.delivery_date = 'Delivery date is required.';
    //   return;
    // }

    // const payload = {
    //   ...this.user_input,
    //   delivery_date: new Date(this.user_input.delivery_date).toISOString()
    // };
//   }
// }

// const otp_verification = {
//   timer: 30,
//   user_input,
//   input_errors,
//   resendDisabled: true,

//   async showView() {
//     const html = await fetch('views/otp-verification.html').then(res => res.text());
//     document.getElementById('otp').innerHTML = html;

//     createApp(
//       otp_verification
//     ).mount();

//     this.startCountdown();
//   },

//   startCountdown() {
//     this.resendDisabled = true;
//     const interval = setInterval(() => {
//       if (this.timer > 1) {
//         this.timer--;
//       } else {
//         this.resendDisabled = false;
//         clearInterval(interval);
//       }
//     }, 1000);
//   },
// }

// const views = {
//   details: {
//     user_input,
//     input_errors,
//     save_to_local_storage,

//     init() {
      // const tomorrow = new Date();
      // tomorrow.setDate(tomorrow.getDate() + 1);

      // const $datepickerEl = document.getElementById('datepicker-custom');

      // // optional options with default values and callback functions
      // const options = {
      //     defaultDatepickerId: null,
      //     autohide: true,
      //     format: 'mm/dd/yyyy',
      //     minDate: tomorrow,
      //     orientation: 'top',
      //     buttons: true,
      // };

      // const instanceOptions = {
      //   id: 'datepicker-custom-example',
      //   override: true
      // };

      // const datepicker = new Datepicker($datepickerEl, options, {});

      // datepicker._datepickerEl.addEventListener('changeDate', (event) => {
      //   const date = event.detail.date; // The selected Date object

      //   function getOrdinal(n) {
      //     const s = ["th", "st", "nd", "rd"];
      //     const v = n % 100;
      //     return n + (s[(v - 20) % 10] || s[v] || s[0]);
      //   }

      //   const days = ["Sunday","Monday","Tuesday","Wednesday","Thursday","Friday","Saturday"];
      //   const months = ["January","February","March","April","May","June","July","August","September","October","November","December"];

      //   const formatted = `${days[date.getDay()]}, ${getOrdinal(date.getDate())} ${months[date.getMonth()]} ${date.getFullYear()}`;

      //   datepickerEl.value = formatted;
      // });

      // flatpickr("#datetime", {
      //   minDate: tomorrow,
      //   dateFormat: "D, d M Y",
      //   formatDate: (date, format, locale) => {
      //     const days = [
      //       "Sunday", "Monday", "Tuesday", "Wednesday",
      //       "Thursday", "Friday", "Saturday"
      //     ];
      //     const months = [
      //       "January", "February", "March", "April", "May", "June",
      //       "July", "August", "September", "October", "November", "December"
      //     ];

      //     const getOrdinal = (n) => {
      //       const s = ["th", "st", "nd", "rd"];
      //       const v = n % 100;
      //       return n + (s[(v - 20) % 10] || s[v] || s[0]);
      //     };

      //     const dayName = days[date.getDay()];
      //     const day = getOrdinal(date.getDate());
      //     const month = months[date.getMonth()];
      //     const year = date.getFullYear();

      //     return `${dayName}, ${day} ${month} ${year}`;
      //   },
      // });
//     },

//     async submit() {
//       if (!user_input.delivery_date) {
//         input_errors.delivery_date = 'Delivery date is required.';
//         return;
//       }

//       const payload = {
//         ...this.user_input,
//         delivery_date: new Date(this.user_input.delivery_date).toISOString()
//       };

//       fakeAxios('/api/success200', {
//         method: 'POST',
//         headers: { 'Content-Type': 'application/json' },
//         data: payload
//       })
//       .then(() => {
//         otp_verification.showView();
//         // load_view('verify');
//       })
//       .catch(err => {
        // if (err.response.status === 400) {
        //   this.input_errors = err.response.data.errors;
        // } else {
        //   document.getElementById('errorModal').classList.remove('hidden');
        // }
//       });
//     }
//   },

//   verify: {
//     timer: 30,
//     resendDisabled: true,

//     user_input,
//     input_errors,
//     pin: Array(6).fill(''),

//     startCountdown() {
//       console.log('started');
//       this.resendDisabled = true;

//       const interval = setInterval(() => {
//         if (this.timer > 1) {
//           this.timer--;
//           console.log("new timer: ", this.timer);
//         } else {
//           this.resendDisabled = false;
//           clearInterval(interval);
//           console.log("Timer finished");
//         }
//       }, 1000);
//     },

//     resend() {
//       if (this.resendDisabled) return;

//       // Call the API or logic and then 

//       this.timer = 30
//       this.startCountdown();
//     },

//     moveFocus(index) {
//       const el = document.getElementById(`code-${index + 1}`);
//       const next = document.getElementById(`code-${index + 2}`);
//       const prev = document.getElementById(`code-${index}`);

//       if (el.value === '' && prev) {
//         prev.focus();
//       } else if (el.value && next) {
//         next.focus();
//       }
//     },

//     handlePaste(event) {
//       event.preventDefault();
//       const paste = (event.clipboardData || window.Clipboard).getData('text');
//       const digits = paste.replace(/\D/g, '').split('');

//       digits.forEach((digit, i) => {
//         if (i < 6) {
//           this.pin[i] = digit;
//           const input = document.getElementById(`code-${i + 1}`);
//           if (input) input.value = digit;
//         }
//       });

//       const next = document.getElementById('code-6');
//       if (next) next.focus();
//     },

//     submit() {
//       const pin = this.pin.join('');
//       if (pin.length !== 6) {
//         input_errors.otp = 'Please enter a valid 6-digit code.';
//         return;
//       }

//       fakeAxios('/api/success200', {
//         method: 'POST',
//         headers: { 'Content-Type': 'application/json' },
//         data: { pin }
//       })
//       .then(() => {
//         document.getElementById('successScreen').classList.remove('hidden');
//       })
//       .catch(err => {
//         if (err.response.status === 400) {
//           alert('Invalid code. Please try again.');
//         } else {
//           document.getElementById('errorModal').classList.remove('hidden');
//         }
//       });
//     }
//   },
// };

// async function load_view(viewName) {
//   const html = await fetch(`views/${viewName}.html`).then(res => res.text());
//   document.getElementById('app').innerHTML = html;

//   const viewLogic = views[viewName];
//   if (viewLogic?.init) viewLogic.init();

//   createApp({
//     details: views.details,
//     message: views.message,
//     verify: views.verify,

//     otp_verification: otp_verification,
//   }).mount();
// }

// async function main() {
//   const saved = localStorage.getItem(STORAGE_KEY);
//   if (saved) {
//     try {
//       Object.assign(user_input, JSON.parse(saved));
//     } catch {}
//   }

//   const html = await fetch('views/modals.html').then(res => res.text());
//   document.getElementById('modals').innerHTML = html;

//   await load_view('details');
// }

// main();







// async function fakeAxios(url, options = {}) {
//   return new Promise((resolve, reject) => {
//     setTimeout(() => {
//       if (url.endsWith('/error500')) {
//         reject({ response: { status: 500, data: null } });
//       } else if (url.endsWith('/success200')) {
//         resolve({
//           data: { message: 'Success' },
//           status: 200,
//           headers: { 'Content-Type': 'application/json' }
//         });
//       } else {
//         const errors = {
//           name: 'Name is required',
//           email: 'Email is required',
//           message: 'Message is required',
//           delivery_date: 'Date must be at least 24 hours in the future'
//         };

//         reject({
//           response: {
//             status: 400,
//             data: { errors },
//             headers: { 'Content-Type': 'application/json' }
//           }
//         });
//       }
//     }, 700);
//   });
// }