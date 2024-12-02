import Model, { attr } from '@ember-data/model';

export default class CardModel extends Model {
  @attr('string') cardName;
  @attr('string') rtype;
  @attr('string') icon;
  @attr('string') color;
}
