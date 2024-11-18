import Controller from '@ember/controller';
import { tracked } from '@glimmer/tracking';
import { action } from '@ember/object';
import { service } from '@ember/service';

export default class Registration extends Controller {
    @tracked username = '';
    @tracked password = '';
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
    }
    
    @action
    async handleRegistration(event) {
        event.preventDefault();
    
        // const formBody = JSON.stringify({
        //     email: this.email,
        //     username: this.username,
        //     password: this.password,
        // });
        
        this.transitionTo('login');
    }
}