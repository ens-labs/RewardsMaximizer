import Model, { attr } from '@ember-data/model';

export default class CardModel extends Model {
  @attr('string') cardName;
  @attr('string') cardBenefits;
  @attr('string') iconClass;
  @attr('string') bgColorClass;
}
