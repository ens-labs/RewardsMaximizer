import Controller from '@ember/controller';
import { tracked } from '@glimmer/tracking';
import { action } from '@ember/object';
import { service } from '@ember/service';

export default class Login extends Controller {
  @tracked email = '';
  @tracked password = '';
  @service router;

  @action
  login(event) {
    event.preventDefault();
    this.router.transitionTo('home');
  }
}
