import { reactive } from 'https://unpkg.com/petite-vue?module';

const isLocal = window.location.hostname === 'localhost';

export const API_BASE_URL = isLocal
  ? 'http://localhost:8080'
  : 'https://futureyou.onrender.com';


export const STORAGE_KEY = 'future-me-form';

export const input_errors = reactive({ });

export const user_input = reactive({
  name: '',
  email: '',
  message: '',
  delivery_date: '',
});

export const modal = reactive({
  showErrorModal: false,
  showSuccessModal: false,

  isVisible: false,

  sendAnother() {
    location.reload();
  },

  askFriend() {
    if (navigator.share) {
      navigator.share({
        url: window.location.href,
        title: 'Write a letter to your future self!',
        text: 'Check out this cool website where you can write a letter to your future self!',
      })
      .then(() => console.log('Thanks for sharing!'))
      .catch(console.error);
    } else {
      alert('Sharing is not supported on this browser.');
    }
  },

  openErrorModal() {
    this.showErrorModal = true;
    setTimeout(() => {
      this.isVisible = true;
    }, 10);
  },

  openSuccessModal() {
    this.showSuccessModal = true;
    setTimeout(() => {
      this.isVisible = true;
    }, 10);
  },

  close() {
    this.isVisible = false;
    setTimeout(() => {
      this.showErrorModal = false;
      this.showSuccessModal = false;
    }, 300);
  }
});