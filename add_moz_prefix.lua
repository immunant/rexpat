-- c2rust-refactor script that prepends the `MOZ_` prefix
-- to all functions that are renamed using `#define`
-- inside Firefox's `expat_config.h`, which we parse here

local Visitor = {}
Visitor.__index = Visitor

function Visitor.new(transform)
    local self = {}
    self.transform = transform
    self.renames = {}

    -- Parse the renames directly from Firefox's `expat_config.h`
    for line in io.lines("expat_config.h") do
        local from, to = line:match("#define%s+([%w_]+)%s+(MOZ_[%w_]+)")
        if from then
            self.renames[from] = to
        end
    end

    setmetatable(self, Visitor)
    return self
end

function Visitor:visit_ident(ident, walk)
    walk(ident)

    local ident_name = ident:get_name()
    local rename = self.renames[ident_name]
    if rename then
        print("Renaming " .. ident_name .. " => " .. rename)
        ident:set_name(rename)
    end
end

refactor:transform(
    function(transform)
        local vis = Visitor.new(transform)
        transform:visit_crate_new(vis)
    end, 1
)

refactor:save_crate()
