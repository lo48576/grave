module Lo48576
module Blogging
  include Nanoc::Helpers::Blogging

  def articles_sorted_by_creation
    blk = -> {
      articles.sort_by { |a| attribute_to_time(a[:created_at]) }.reverse
    }

    if @items.frozen?
      @article_items_sorted_by_creation ||= blk.call
    else
      blk.call
    end
  end

  def articles_sorted_by_publish
    blk = -> {
      articles.sort_by { |a|
        attribute_to_time(a[:published_at]) || attribute_to_time(a[:created_at])
      }.reverse
    }

    if @items.frozen?
      @article_items_sorted_by_publish ||= blk.call
    else
      blk.call
    end
  end

  def articles_sorted_by_update
    blk = -> {
      articles.sort_by { |a|
        attribute_to_time(a[:updated_at]) || attribute_to_time(a[:published_at]) || attribute_to_time(a[:created_at])
      }.reverse
    }

    if @items.frozen?
      @article_items_sorted_by_update ||= blk.call
    else
      blk.call
    end
  end
end
end
