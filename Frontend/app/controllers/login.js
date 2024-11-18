import Controller from '@ember/controller';
import { tracked } from '@glimmer/tracking';
import { action } from '@ember/object';
import { service } from '@ember/service';

export default class Login extends Controller {
  @tracked username = '';
  @tracked password = '';
  @service router;

  @action
  setUsername(event) {
    this.username = event.target.value;
  }

  @action
  setPassword(event) {
    this.password = event.target.value;
  }

  @action
  async handleLogin(event) {
    event.preventDefault();

    const formBody = JSON.stringify({
      username: "username",
      password: "password",
    });

    try {
      let response = await fetch('http://localhost:8080/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: formBody,
      });

      if (!response.ok) {
        throw new Error('Login failed');
      }

      let data = await response.json();
      console.log('Login successful:', data);
      this.transitionTo('home'); 
    } 
    catch (error) {
      this.errorMessage = error.message || 'Error logging in';
    }
  }
}
