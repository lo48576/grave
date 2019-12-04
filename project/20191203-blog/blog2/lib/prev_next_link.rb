module Lo48576
module PrevNextLink
  def prev_item(items, target: @item)
    if items
      prev_index = items.index(target) - 1
      if prev_index >= 0
        items[prev_index]
      end
    end
  end

  def next_item(items, target: @item)
    if items
      next_index = items.index(target) + 1
      if next_index < items.size
        items[next_index]
      end
    end
  end

  def prev_link(items, prefix: nil, suffix: nil, target: @item, options: {})
    if prev_page = prev_item(items, target: target)
      link_to "#{prefix}#{prev_page[:title]}#{suffix}", prev_page, options
    end
  end

  def next_link(items, prefix: nil, suffix: nil, target: @item, options: {})
    if next_page = next_item(items, target: target)
        link_to "#{prefix}#{next_page[:title]}#{suffix}", next_page, options
    end
  end

  def older_article
    next_link sorted_articles, prefix: '<small>older &#xBB;</small> '
  end

  def newer_article
    prev_link sorted_articles, suffix: ' <small>&#xAB; newer</small>'
  end
end
end

include Lo48576::PrevNextLink
