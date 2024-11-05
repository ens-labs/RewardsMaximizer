// Ember route for the login form
import Route from '@ember/routing/route';

export default class LoginRoute extends Route {
  model() {
    return {};
  }

  actions: {
    login() {
      let { email, password } = this.controller.getProperties('email', 'password');
      // Perform login logic here, such as making an API call
      console.log(`Logging in with: ${email}, ${password}`);
    }
  }
}