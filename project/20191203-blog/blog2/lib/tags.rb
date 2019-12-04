module Lo48576
module Tags
  require 'rgl/adjacency';
  require 'rgl/topsort';
  require 'rgl/transitivity';

  # HTMLEscape: `h()`
  include Nanoc::Helpers::HTMLEscape

  def tags
    blk = -> items {
      Tags.new(items)
    }
    if @items.frozen?
      @tags ||= blk.call(@items)
    else
      blk.call(@items)
    end
  end

  class Tags
    def initialize(items)
      @items = items
      @graph = RGL::DirectedAdjacencyGraph.new
      tag_items = @items.find_all('/tags/data/*.adoc').select {|item| item[:kind] == 'tag'}
      @graph.add_vertices(*tag_items.collect {|i| Tags.from_item(i) })
      tag_items.each do |item|
        tag = Tags.from_item(item)
        raise "Unknown tag: `#{tag}` for item `#{item.identifier}`" unless @graph.has_vertex?(tag)
        next unless children = item[:imply]
        if children.is_a? Enumerable
          children.each do |child|
            raise "Unknown tag: `#{child}` implied by `#{tag}`" unless @graph.has_vertex?(child)
          end
          children.reject {|child| tag == child }.each do |child|
            @graph.add_edge(tag, child)
          end
        else
          raise "Unknown tag: `#{children}` implied by `#{tag}`" unless @graph.has_vertex?(children)
          @graph.add_edge(tag, children) unless tag == children
        end
      end

      raise "Acyclic graph is not allowed for tags" unless @graph.acyclic?
      @graph_indirect = @graph.transitive_closure
      @graph.each do |v|
        @graph_indirect.remove_edge(v, v)
      end
    end

    def each(&block)
      @graph.each(&block)
    end

    def sort(&block)
      @graph.sort(&block)
    end

    def sort_by(&block)
      @graph.sort_by(&block)
    end

    def parents(tag)
      if tag.is_a? Enumerable
        Set.new(tag.flat_map {|tag| parents(tag).to_a})
      else
        raise "Unknown tag `#{tag}`" unless @graph.has_vertex?(tag)
        @graph_indirect.each_adjacent(tag)
      end
    end

    def children(tag)
      if tag.is_a? Enumerable
        Set.new(tag.flat_map {|tag| children(tag).to_a})
      else
        raise "Unknown tag `#{tag}`" unless @graph.has_vertex?(tag)
        @graph.edges.select {|e| e.target == tag }.map {|e| e.source }
      end
    end

    def ancestors(tag)
      if tag.is_a? Enumerable
        Set.new(tag.flat_map {|tag| ancestors(tag).to_a})
      else
        raise "Unknown tag `#{tag}`" unless @graph.has_vertex?(tag)
        @graph_indirect.each_adjacent(tag)
      end
    end

    def descendants(tag)
      if tag.is_a? Enumerable
        Set.new(tag.flat_map {|tag| descendants(tag).to_a})
      else
        raise "Unknown tag `#{tag}`" unless @graph.has_vertex?(tag)
        @graph_indirect.edges.select {|e| e.target == tag }.map {|e| e.source }
      end
    end

    def descendants_and_self(tag)
      if tag.is_a? Enumerable
        Set.new(tag.flat_map {|tag| descendants_and_self(tag).to_a})
      else
        raise "Unknown tag `#{tag}`" unless @graph.has_vertex?(tag)
        Set.new(@graph_indirect.edges.select {|e| e.target == tag }.map {|e| e.source }.to_a).add(tag)
      end
    end

    def indirect_ancestors(tag)
      Set.new(ancestors(tag)) - parents(tag)
    end

    def indirect_descendants(tag)
      Set.new(descendants(tag)) - children(tag)
    end

    def item(tag)
      raise "Unknown tag `#{tag}`: graph=#{@graph}" unless @graph.has_vertex?(tag)
      @items[Tags.path(tag)]
    end

    def articles_direct(tag)
      raise "Unknown tag `#{tag}`" unless @graph.has_vertex?(tag)
      @items.select {|item| item[:tags]&.include?(tag) || false}
    end

    def articles_indirect(tag)
      raise "Unknown tag `#{tag}`" unless @graph.has_vertex?(tag)
      @items.reject {|item| ((item[:tags] || []) & descendants(tag)).empty?}.
        reject {|item| item[:tags].include?(tag)}
    end

    def self.path(tag)
      "/tags/data/#{tag}.adoc"
    end

    def self.from_item(item)
      %r[/tags/data/(.+)\.adoc].match(item.identifier)[1]
    end
  end
end
end

include Lo48576::Tags
