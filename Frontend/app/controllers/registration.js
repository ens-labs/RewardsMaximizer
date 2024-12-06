import Controller from '@ember/controller';
import { tracked } from '@glimmer/tracking';
import { action } from '@ember/object';
import { service } from '@ember/service';

export default class Registration extends Controller {
  @tracked email = '';
  @tracked username = '';
  @tracked password = '';
  @tracked passwordTwo = '';
  @tracked errorMessage = '';
  passwordError = false;
  @service router;

  @action
  setEmail(event) {
    this.email = event.target.value;
  }

  @action
  setUsername(event) {
    this.username = event.target.value;
  }

  @action
  setPassword(event) {
    this.password = event.target.value;
    const passwordRegex = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$/;

    if (this.password === '') {
      this.passwordError = false;
      this.errorMessage = '';
    } 
    else {
      const isValidPassword = passwordRegex.test(this.password);
      this.passwordError = !isValidPassword;
      this.errorMessage = isValidPassword ? '' : 'Password must contain at least 8 characters, one uppercase letter, one lowercase letter, one number, and one special character.';
    }
  }

  @action
  setPasswordTwo(event) {
    this.passwordTwo = event.target.value;
    
    if (this.passwordTwo === '') {
      this.errorMessage = '';
    } 
    else if (this.password !== this.passwordTwo) {
      this.errorMessage = 'Passwords do not match!';
      this.passwordError = true;
    } 
    else {
      this.errorMessage = '';
      this.passwordError = false;
    }
  }

  @action
  async handleRegistration(event) {
    event.preventDefault();

    // If there are no errors with the password, send a POST request to the server
    // Server side signup TBD
    // if (this.passwordError === false) {

    //   try{
    //     let response = await fetch('http://localhost:8080/signup', {
    //       method: 'POST',
    //       headers: {
    //         'Content-Type': 'application/json',
    //       },
    //       body: JSON.stringify({
    //         email: this.email,
    //         username: this.username,
    //         password: this.password,
    //       }),
    //     });

    //     if (!response.ok) {
    //       throw new Error('Registration failed');
    //     }
    //     this.router.transitionTo('login');
    //   } 
    //   catch (error) {
    //     this.errorMessage = error.message;
    //   }
    // }
  }

  @action
  togglePasswordVisibility(event) {
    const passwordField = event.target.closest('.input-group').querySelector('input');
    passwordField.type = passwordField.type === 'password' ? 'text' : 'password';
    event.target.classList.toggle('fa-eye');
    event.target.classList.toggle('fa-eye-slash');
  }

  // Return to login page
  @action
  back() {
    this.router.transitionTo('login');
  }
}
